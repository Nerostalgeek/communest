use crate::config::db::{get_db_connection, DatabaseError, DbPool};
use crate::models::user::AuthRequest;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::utils::jwt::{self, Claims};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use std::sync::Arc;

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
                if chrono::Utc::now().naive_utc() > expiration {
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
}
#[derive(Debug)]
pub enum AuthServiceError {
    UserNotFound,
    IncorrectPassword,
    AccountLocked,
    AccountNotActivated,
    ActivationExpired,
    InternalServerError,
    PasswordVerificationError,
    JwtError(String),
    Other(String),
}

impl From<DieselError> for AuthServiceError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => AuthServiceError::UserNotFound,
            DieselError::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    AuthServiceError::Other(info.message().to_string())
                }
                _ => AuthServiceError::Other(info.message().to_string()),
            },
            _ => AuthServiceError::InternalServerError,
        }
    }
}

impl From<DatabaseError> for AuthServiceError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::ConnectionPoolError => AuthServiceError::InternalServerError,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AuthServiceError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        AuthServiceError::JwtError(error.to_string())
    }
}

impl From<argon2::password_hash::Error> for AuthServiceError {
    fn from(error: argon2::password_hash::Error) -> Self {
        AuthServiceError::PasswordVerificationError
    }
}
