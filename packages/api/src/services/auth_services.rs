use crate::config::db::{get_db_connection, DatabaseError, DbPool};
use crate::models::user::{
    ActivateAccountRequest, AuthRequest, PasswordResetRequest, ValidateResetPasswordRequest,
};
use crate::schema::users::dsl::*;
use crate::utils::jwt::{self};
use crate::{
    models::user::{CreateUserRequest, NewUser, User},
    schema::users,
    services::email_services::{send_email, EmailContext, EmailServiceError, EmailType},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use log::{error, info, warn};
use sendgrid::SGClient;
use std::sync::Arc;
use thiserror::Error as ThisError;
use uuid::Uuid;

pub struct AuthService {
    pool: Arc<DbPool>,
}

impl AuthService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        AuthService { pool }
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
        sg_client: Arc<SGClient>,
    ) -> Result<User, AuthServiceError> {
        info!(
            "Attempting to create a new user with email: {}",
            request.email
        );
        let mut conn = get_db_connection(self.pool.clone()).await.map_err(|_| {
            error!("Failed to get DB connection");
            AuthServiceError::DatabaseConnectionPoolError
        })?;

        let generated_password_hash =
            AuthService::hash_password(&request.password).map_err(|e| {
                error!("Password hashing failed: {:?}", e);
                AuthServiceError::InternalServerError
            })?;

        let generated_verification_token = Uuid::new_v4();
        let new_user = NewUser {
            last_name: request.last_name,
            first_name: request.first_name,
            email: request.email.clone(),
            phone_number: request.phone_number,
            password_hash: generated_password_hash,
            account_activation_token: generated_verification_token,
            account_activation_token_expires_at: Utc::now() + Duration::hours(24),
        };

        let created_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .map_err(AuthServiceError::from)?;

        // Create the email context for sending the activation email.
        let email_context = EmailContext {
            recipient: request.email.clone(),
            token: generated_verification_token.to_string(),
        };

        match send_email(sg_client, EmailType::Activation, &email_context).await {
            Ok(_) => info!("Activation email sent successfully."),
            Err(e) => warn!("Failed to send activation email: {:?}", e),
        }

        Ok(created_user)
    }

    pub async fn authenticate(
        &self,
        auth_details: AuthRequest,
        jwt_secret: &[u8], // Use &[u8] to match the JWT utility expectations
    ) -> Result<String, AuthServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await?;

        // Correct use of `optional()` to handle possible absence of the user
        let user = users
            .filter(email.eq(&auth_details.email))
            .first::<User>(&mut conn)
            .optional()?;

        let user = match user {
            Some(user) => user,
            None => return Err(AuthServiceError::UserNotFound),
        };

        // Verifying the password
        let password_verified =
            AuthService::verify_password(&user.password_hash, &auth_details.password)?;

        if password_verified {
            if !user.is_activated {
                return Err(AuthServiceError::AccountNotActivated);
            }

            if let Some(expiration) = user.account_activation_token_expires_at {
                // Here we compare DateTime<Utc> with the current Utc time directly
                if Utc::now() > expiration {
                    return Err(AuthServiceError::ActivationExpired);
                }
            }
            let token: String = jwt::generate_jwt(&user.id.to_string(), jwt_secret)?;
            Ok(token)
        } else {
            Err(AuthServiceError::IncorrectPassword)
        }
    }

    pub async fn activate_account(
        &self,
        request: ActivateAccountRequest,
    ) -> Result<(), AuthServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await.map_err(|_| {
            error!("Failed to get DB connection");
            AuthServiceError::DatabaseConnectionPoolError
        })?;

        let user = users::table
            .filter(users::account_activation_token.eq(request.token))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|_| AuthServiceError::QueryError)?;

        match user {
            Some(user) => {
                if let Some(expiration) = user.account_activation_token_expires_at {
                    if Utc::now() > expiration {
                        return Err(AuthServiceError::TokenExpired);
                    }
                } else {
                    return Err(AuthServiceError::InvalidToken);
                }

                diesel::update(users::table.find(user.id))
                    .set((
                        users::is_activated.eq(true),
                        users::account_activation_token.eq::<Option<Uuid>>(None), // Invalidate the token
                        users::account_activation_token_expires_at
                            .eq::<Option<NaiveDateTime>>(None),
                    ))
                    .execute(&mut conn)
                    .map_err(|_| AuthServiceError::QueryError)?;

                Ok(())
            }
            None => Err(AuthServiceError::InvalidToken),
        }
    }

    pub fn verify_password(hash: &str, password: &str) -> Result<bool, AuthServiceError> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();

        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|_| AuthServiceError::PasswordVerificationError)
    }

    pub fn hash_password(password: &str) -> Result<String, AuthServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())?)
    }

    pub async fn request_password_reset(
        &self,
        request: PasswordResetRequest,
        sg_client: Arc<SGClient>,
    ) -> Result<(), AuthServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await.map_err(|_| {
            error!("Failed to get DB connection");
            AuthServiceError::DatabaseConnectionPoolError
        })?;

        // Attempt to find the user by email
        let found_user = users::table
            .filter(users::email.eq(&request.email))
            .first::<User>(&mut conn)
            .optional()?; // Notice the use of optional to handle not found case

        // If user is found, generate the password reset token
        if let Some(user) = found_user {
            let generated_password_token = Uuid::new_v4();
            let generated_password_token_expiration = Utc::now() + Duration::hours(24);

            // Update the user with the new token and its expiration
            diesel::update(users::table.find(user.id))
                .set((
                    users::password_reset_token.eq(Some(generated_password_token)),
                    users::password_reset_expires_at.eq(Some(generated_password_token_expiration)),
                ))
                .execute(&mut conn)?;

            let email_context = EmailContext {
                recipient: request.email.clone(),
                token: generated_password_token.to_string(),
            };

            match send_email(sg_client, EmailType::PasswordReset, &email_context).await {
                Ok(_) => info!("Password reset email sent successfully."),
                Err(e) => warn!("Failed to send Password reset email: {:?}", e),
            }
            // Here you should send the password reset email to the user with the token embedded in a link

            Ok(())
        } else {
            // If user is not found, you can choose to return Ok to prevent email enumeration
            // or return an error if you want the client to know that the email was not found
            Ok(())
        }
    }

    pub async fn validate_password_reset(
        &self,
        request: ValidateResetPasswordRequest,
    ) -> Result<(), AuthServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await.map_err(|_| {
            error!("Failed to get DB connection");
            AuthServiceError::DatabaseConnectionPoolError
        })?;

        // Assuming `users` is the table, and `password_reset_token` & `password_reset_expires_at` are columns
        let user = users::table
            .filter(users::password_reset_token.eq(request.token))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|_| AuthServiceError::QueryError)?;

        match user {
            Some(user) => {
                // Check if the token is expired
                if let Some(expiration) = user.password_reset_expires_at {
                    if Utc::now() > expiration {
                        return Err(AuthServiceError::TokenExpired);
                    }
                } else {
                    // Token was found but no expiration was set, which should not happen
                    return Err(AuthServiceError::InvalidToken);
                }

                // Set the new password for the user (ensure you hash the password!)
                let hashed_password = Self::hash_password(&request.new_password)
                    .map_err(|_| AuthServiceError::PasswordHashError)?;
                diesel::update(users::table.find(user.id))
                    .set((
                        users::password_hash.eq(hashed_password),
                        users::password_reset_token.eq::<Option<Uuid>>(None), // Invalidate the token
                        users::password_reset_expires_at.eq::<Option<NaiveDateTime>>(None),
                    ))
                    .execute(&mut conn)
                    .map_err(|_| AuthServiceError::QueryError)?;

                Ok(())
            }
            None => Err(AuthServiceError::InvalidToken),
        }
    }
}
#[derive(ThisError, Debug)]
pub enum AuthServiceError {
    #[error("User not found")]
    UserNotFound,
    #[error("Wrong credentials")]
    IncorrectPassword,
    #[error("Account is not activated")]
    AccountNotActivated,
    #[error("Account activation date expired")]
    ActivationExpired,
    #[error("Password verification error")]
    PasswordVerificationError,
    #[error("database connection pool error")]
    DatabaseConnectionPoolError,
    #[error("Database error")]
    DatabaseError(#[from] DieselError),
    #[error("internal server error")]
    InternalServerError,
    #[error("validation error: {0}")]
    ValidationError(String),
    #[error("query error")]
    QueryError,
    #[error(transparent)]
    EmailError(#[from] EmailServiceError),
    #[error("Ressource not found")]
    ResourceNotFound, // ... other errors ...
    #[error("Error with JWT: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Other error : {0}")]
    Other(String),
    #[error("Password hash error")]
    PasswordHashError,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
}

// Remove the manual From<DieselError> implementation to avoid conflict with thiserror's automatic implementation.

impl From<DatabaseError> for AuthServiceError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::ConnectionPoolError => AuthServiceError::InternalServerError,
        }
    }
}

impl From<argon2::password_hash::Error> for AuthServiceError {
    fn from(error: argon2::password_hash::Error) -> Self {
        AuthServiceError::PasswordVerificationError
    }
}
