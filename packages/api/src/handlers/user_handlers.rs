use crate::config::db::DbPool;
use crate::models::user::CreateUserRequest;
use crate::services::user_service::{UserService, UserServiceError};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use std::sync::Arc;
pub async fn register_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<CreateUserRequest>,
) -> impl Responder {
    info!("Received request to register a new user: {:?}", new_user);
    // This syntax might be used to emphasize that we're cloning the Arc, not the data inside it, but in practice, we can just use Arc::clone(&pool) for clarity.
    let user_service = UserService::new(Arc::clone(&pool));

    match user_service.create_user(new_user.into_inner()).await {
        Ok(user) => {
            info!("User registered successfully: {}", user.id);
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            error!("Error during user registration: {:?}", e);

            // Respond with detailed message in development or generic message in production
            let is_dev = cfg!(debug_assertions); // True if in debug mode
            let error_message = if is_dev {
                format!("{}", e) // In development, show detailed errors
            } else {
                "An internal server error occurred".to_string() // In production, show generic error message
            };

            match e {
                UserServiceError::DatabaseConnectionPoolError => {
                    HttpResponse::ServiceUnavailable().body(error_message)
                }
                UserServiceError::ResourceNotFound => HttpResponse::NotFound().body(error_message),
                UserServiceError::DatabaseError(_) => {
                    HttpResponse::BadRequest().body(error_message)
                }
                _ => HttpResponse::InternalServerError().body(error_message),
            }
        }
    }
}
