use crate::config::db::{get_db_connection, DatabaseError, DbPool};
use crate::models::user::AuthRequest;
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

use chrono::{Duration, Utc};
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

            if let Some(expiration) = user.activation_expires_at {
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

    pub fn verify_password(hash: &str, password: &str) -> Result<bool, AuthServiceError> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();

        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|_| AuthServiceError::PasswordVerificationError)
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
            verification_token: generated_verification_token,
            token_expires_at: Utc::now() + Duration::hours(24),
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

    pub fn hash_password(password: &str) -> Result<String, AuthServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())?)
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
    #[error(transparent)]
    EmailError(#[from] EmailServiceError),
    #[error("Ressource not found")]
    ResourceNotFound, // ... other errors ...
    #[error("Error with JWT: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Other error : {0}")]
    Other(String),
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
