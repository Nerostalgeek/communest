use std::sync::Arc;

use crate::config::db::DbPool;
use crate::models::user::CreateUserRequest;
use crate::services::user_service::{UserService, UserServiceError};
use actix_web::{web, HttpResponse, Responder};

pub async fn register_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<CreateUserRequest>,
) -> impl Responder {
    // This syntax might be used to emphasize that we're cloning the Arc, not the data inside it, but in practice, we can just use Arc::clone(&pool) for clarity.
    let user_service = UserService::new(Arc::clone(&(*pool)));
    match user_service.create_user(new_user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => match e {
            UserServiceError::DatabaseConnectionPoolError => {
                HttpResponse::ServiceUnavailable().body("Database connection pool error")
            }
            UserServiceError::ResourceNotFound => {
                HttpResponse::NotFound().body("Resource not found")
            }
            UserServiceError::DatabaseError(err) => HttpResponse::BadRequest().body(err),
            _ => HttpResponse::InternalServerError().body("Internal server error"),
        },
    }
}
