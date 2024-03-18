use crate::config::db::DbPool;
use crate::models::user::{
    ActivateAccountRequest, AuthRequest, AuthResponse, TokenRefreshRequest, TokenResponse,
    ValidateResetPasswordRequest,
};
use crate::models::user::{CreateUserRequest, PasswordResetRequest};
use crate::services::auth_services::{AuthService, AuthServiceError};
use crate::AppConfig;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};

use std::sync::Arc; // Ensure AppConfig is accessible

pub async fn register_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<CreateUserRequest>,
    config: web::Data<AppConfig>, // Pass in the API key for SendGrid
) -> impl Responder {
    info!("Received request to register a new user: {:?}", new_user);
    let auth_service = AuthService::new(Arc::clone(&pool));

    match auth_service
        .create_user(new_user.into_inner(), config.sendgrid_client.clone())
        .await
    {
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
                AuthServiceError::DatabaseConnectionPoolError => {
                    HttpResponse::ServiceUnavailable().body(error_message)
                }
                AuthServiceError::ResourceNotFound => HttpResponse::NotFound().body(error_message),
                AuthServiceError::DatabaseError(_) => {
                    HttpResponse::BadRequest().body(error_message)
                }
                _ => HttpResponse::InternalServerError().body(error_message),
            }
        }
    }
}

pub async fn login_user(
    pool: web::Data<DbPool>,
    auth_details: web::Json<AuthRequest>,
    app_config: web::Data<AppConfig>, // Extract AppConfig to access jwt_secret
) -> impl Responder {
    let auth_service = AuthService::new(Arc::clone(&pool));

    match auth_service
        .authenticate(auth_details.into_inner(), app_config.jwt_secret.as_bytes())
        .await
    {
        Ok(token) => HttpResponse::Ok().json(AuthResponse { token }),
        Err(e) => match e {
            AuthServiceError::UserNotFound => HttpResponse::NotFound().finish(),
            AuthServiceError::IncorrectPassword => HttpResponse::Unauthorized().finish(),
            AuthServiceError::AccountNotActivated => {
                HttpResponse::Forbidden().body("Account is not activated.")
            }
            AuthServiceError::ActivationExpired => {
                HttpResponse::Forbidden().body("Activation has expired.")
            }
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

pub async fn activate_account(
    pool: web::Data<DbPool>,
    request: web::Path<ActivateAccountRequest>,
) -> impl Responder {
    let service = AuthService::new(Arc::clone(&pool));

    match service.activate_account(request.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Account has been activated."),
        Err(e) => {
            log::error!("Failed to activate account: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn request_password_reset(
    pool: web::Data<DbPool>,
    request: web::Json<PasswordResetRequest>,
    config: web::Data<AppConfig>,
) -> impl Responder {
    let auth_service = AuthService::new(Arc::clone(&pool));

    // Process the password reset request and log any errors internally
    let result = auth_service
        .request_password_reset(request.into_inner(), config.sendgrid_client.clone())
        .await;

    match result {
        Ok(_) => {
            // If the operation was successful, you might want to log that as well, or just do nothing.
            log::info!("Password reset request initiated successfully.");
        }
        Err(e) => {
            // Log the error internally for monitoring or auditing purposes
            log::error!("Error initiating password reset: {:?}", e);
        }
    }

    // Respond with HTTP 200 OK regardless of the outcome to avoid leaking information
    HttpResponse::Ok().finish()
}

// Handler to validate the password reset token and set the new password
pub async fn confirm_password_reset(
    pool: web::Data<DbPool>,
    request: web::Json<ValidateResetPasswordRequest>, // This struct needs to be defined
) -> impl Responder {
    let service = AuthService::new(Arc::clone(&pool));
    match service.validate_password_reset(request.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Password has been reset successfully."),
        Err(e) => {
            log::error!("Failed to reset password: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn refresh_jtw_token(
    pool: web::Data<DbPool>,
    request: web::Json<TokenRefreshRequest>,
) -> impl Responder {
    let service = AuthService::new(Arc::clone(&pool));
    match service
        .refresh_jwt_token(request.refresh_token.into_inner())
        .await
    {
        Ok(token) => HttpResponse::Ok().json(TokenResponse { token }),
        Err(e) => match e {
            AuthServiceError::InvalidRefreshToken => HttpResponse::Unauthorized().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
