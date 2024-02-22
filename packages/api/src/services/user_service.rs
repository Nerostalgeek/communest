use crate::{
    config::db::{get_db_connection, DbPool},
    models::user::{NewUser, User},
    schema::users,
};

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use std::sync::Arc;

pub struct UserService {
    pool: Arc<DbPool>, // Use Arc for thread-safe sharing
}

impl UserService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        UserService { pool }
    }

    pub async fn create_user(&self, mut new_user: NewUser) -> Result<User, UserServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await?;

        let password_hash = UserService::hash_password(&new_user.password_hash)
            .map_err(|_| UserServiceError::InternalServerError)?;

        new_user.password_hash = password_hash;

        // Insert the new user into the database
        let created_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
            .map_err(|e| UserServiceError::from(e))?;

        Ok(created_user)
    }

    // pub async fn login(&self, mut credentials: LoginRequest) -> Result<AuthResult, AuthError> {}

    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default(); // Utilise la configuration par défaut d'Argon2

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    // pub fn verify_password(
    //     hash: &str,
    //     password: &str,
    // ) -> Result<bool, argon2::password_hash::Error> {
    //     let parsed_hash = PasswordHash::new(hash)?;
    //     let argon2 = Argon2::default();

    //     Ok(argon2
    //         .verify_password(password.as_bytes(), &parsed_hash)
    //         .is_ok())
    // }
}

pub enum UserServiceError {
    DatabaseConnectionPoolError,
    DatabaseError(String),
    InternalServerError,
    ValidationError(String),
    ResourceNotFound,
}

impl From<DieselError> for UserServiceError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => UserServiceError::ResourceNotFound,
            DieselError::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    UserServiceError::ValidationError(info.message().to_string())
                }
                _ => UserServiceError::DatabaseError(info.message().to_string()),
            },
            _ => UserServiceError::InternalServerError,
        }
    }
}

// pub enum AuthResult {
//     Success { user_id: i32, token: String },
// }

// pub enum AuthError {
//     UserNotFound,
//     IncorrectPassword,
//     AccountLocked,
//     AccountNotActivated,
//     ActivationExpired,
//     Other(String),
// }
