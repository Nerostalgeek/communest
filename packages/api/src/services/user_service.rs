use crate::config::smtp::SmtpConfig;
use crate::{
    config::db::{get_db_connection, DatabaseError, DbPool},
    models::user::{CreateUserRequest, NewUser, User},
    schema::users,
    services::email_services::{send_email, EmailContext, EmailServiceError, EmailType},
};
use actix_web::web;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use log::{error, info, warn};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

pub struct UserService {
    pool: Arc<DbPool>,
}

impl UserService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        UserService { pool }
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
        smtp_config: web::Data<SmtpConfig>,
    ) -> Result<User, UserServiceError> {
        info!(
            "Attempting to create a new user with email: {}",
            request.email
        );
        let mut conn = get_db_connection(self.pool.clone()).await.map_err(|_| {
            error!("Failed to get DB connection");
            UserServiceError::DatabaseConnectionPoolError
        })?;

        let password_hash = UserService::hash_password(&request.password).map_err(|e| {
            error!("Password hashing failed: {:?}", e);
            UserServiceError::InternalServerError
        })?;

        let verification_token = Uuid::new_v4();
        let new_user = NewUser {
            last_name: request.last_name,
            first_name: request.first_name,
            email: request.email.clone(),
            phone_number: request.phone_number,
            password_hash,
            verification_token,
            token_expires_at: Utc::now() + Duration::hours(24),
        };

        let created_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .map_err(UserServiceError::from)?;

        // Create the email context for sending the activation email.
        let email_context = EmailContext {
            recipient: request.email.clone(),
            token: verification_token.to_string(),
        };

        // Send the activation email using the email service.
        if let Err(e) = send_email(smtp_config, EmailType::Activation, &email_context).await {
            warn!("Failed to send activation email: {:?}", e);
        }

        Ok(created_user)
    }

    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    }
}

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("database connection pool error")]
    DatabaseConnectionPoolError,
    #[error("database error")]
    DatabaseError(#[from] DieselError),
    #[error("internal server error")]
    InternalServerError,
    #[error("validation error: {0}")]
    ValidationError(String),
    #[error("error sending email: {0}")]
    EmailError(String),
    #[error("internal server error")]
    ResourceNotFound, // ... other errors ...
}

// Remove the manual From<DieselError> implementation to avoid conflict with thiserror's automatic implementation.

impl From<DatabaseError> for UserServiceError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::ConnectionPoolError => UserServiceError::InternalServerError,
        }
    }
}

impl From<EmailServiceError> for UserServiceError {
    fn from(error: EmailServiceError) -> Self {
        match error {
            EmailServiceError::SendError(msg) => UserServiceError::EmailError(msg),
            // Handle other cases as necessary.
        }
    }
}
