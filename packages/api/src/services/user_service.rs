use crate::{
    config::db::{get_db_connection, DatabaseError, DbPool},
    models::user::{CreateUserRequest, User},
    schema::users,
};
use actix_web::HttpResponse;

use diesel::result::Error as DieselError;

use diesel::prelude::*;
use log::error;
use serde_json::json;
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

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, UserServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await?;
        users::table
            .find(user_id)
            .get_result::<User>(&mut conn)
            .map_err(Into::into)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, UserServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await?;
        users::table.load::<User>(&mut conn).map_err(Into::into)
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        user: CreateUserRequest,
    ) -> Result<User, UserServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await?;
        let updated_user = diesel::update(users::table.find(user_id))
            .set((
                users::last_name.eq(user.last_name),
                users::first_name.eq(user.first_name),
                // Update other fields as necessary
            ))
            .get_result::<User>(&mut conn)?;

        Ok(updated_user)
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<usize, UserServiceError> {
        let mut conn = get_db_connection(self.pool.clone()).await?;
        let user_deleted = diesel::delete(users::table.find(user_id)).execute(&mut conn)?;
        Ok(user_deleted)
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
    #[error("user not found")]
    UserNotFound,
}

impl From<DatabaseError> for UserServiceError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::ConnectionPoolError => UserServiceError::DatabaseConnectionPoolError,
            _ => UserServiceError::DatabaseError(DieselError::RollbackTransaction),
        }
    }
}

impl actix_web::ResponseError for UserServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(json!({ "error": self.to_string() }))
    }
}
