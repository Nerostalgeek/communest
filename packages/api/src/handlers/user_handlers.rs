use std::sync::Arc;

use crate::config::db::DbPool;
use crate::models::user::NewUser;
use crate::services::user_service::{UserService, UserServiceError};
use actix_web::{web, HttpResponse, Responder};

pub async fn register_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
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

// async fn login_user(login_data: web::Json<LoginRequest>) -> impl Responder {
//     // Assuming you've retrieved the stored password hash for the user from the database
//     let stored_password_hash = get_stored_password_hash(&login_data.username);

//     let password_is_valid =
//         match UserService::verify_password(&stored_password_hash, &login_data.password) {
//             Ok(valid) => valid,
//             Err(e) => return HttpResponse::InternalServerError().json("Error verifying password"),
//         };

//     if password_is_valid {
//         HttpResponse::Ok().json("User authenticated successfully")
//     } else {
//         HttpResponse::Unauthorized().json("Invalid username or password")
//     }
// }
