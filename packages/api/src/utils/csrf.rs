use actix_web::dev::ServiceRequest;
use uuid::Uuid;

/// Generates a CSRF token to be sent to the client.
pub fn generate_csrf_token() -> String {
    // Here we're using a simple UUID for the CSRF token
    // You might use another method for generating a token
    Uuid::new_v4().to_string()
}

use actix_web::cookie::Cookie;

/// Validates the CSRF token from the request.
pub async fn validate_csrf_token(req: &str) -> bool {
    if let Some(csrf_cookie) = req.cookie("X-CSRF-Token") {
        if let Some(csrf_header) = req.headers().get("X-CSRF-Token") {
            return csrf_cookie.value() == csrf_header.to_str().unwrap_or("");
        }
    }
    false
}
