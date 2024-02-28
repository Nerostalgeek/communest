use crate::models::user::{AuthRequest, AuthResponse};
use crate::services::auth_services::{AuthService, AuthServiceError};
use crate::AppConfig;
use actix_web::{web, HttpResponse, Responder}; // Ensure AppConfig is accessible

pub async fn login_user(
    auth_details: web::Json<AuthRequest>,
    auth_service: web::Data<AuthService>,
    app_config: web::Data<AppConfig>, // Extract AppConfig to access jwt_secret
) -> impl Responder {
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
