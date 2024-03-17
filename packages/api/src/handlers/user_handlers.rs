use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::CreateUserRequest;
use crate::services::user_service::{UserService, UserServiceError};
// This struct could be in a shared module if it's used across multiple handlers.
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

pub async fn get_user(
    user_id: web::Path<Uuid>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    match user_service.get_user_by_id(*user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(UserServiceError::UserNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "User not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn get_users(user_service: web::Data<UserService>) -> impl Responder {
    match user_service.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

pub async fn update_user(
    user_id: web::Path<Uuid>,
    user: web::Json<CreateUserRequest>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    match user_service.update_user(*user_id, user.into_inner()).await {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(UserServiceError::UserNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "User not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}

// Handler for deleting a user
pub async fn delete_user(
    user_id: web::Path<Uuid>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    match user_service.delete_user(*user_id).await {
        Ok(_) => HttpResponse::Ok().json(ErrorResponse {
            error: "User deleted successfully".to_string(),
        }),
        Err(UserServiceError::UserNotFound) => HttpResponse::NotFound().json(ErrorResponse {
            error: "User not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Internal server error".to_string(),
        }),
    }
}
