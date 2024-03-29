use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub csrf_token: String,
}

pub fn generate_jwt(user_id: &str, secret: &[u8]) -> JwtResult<String> {
    // Use try_hours and properly handle the Result
    let expiration_duration = Duration::try_hours(24).expect("Invalid duration for JWT expiration");
    let expiration = Utc::now() + expiration_duration;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration.timestamp() as usize,
        csrf_token: Uuid::new_v4().to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn verify_jwt(token: &str, secret: &[u8]) -> JwtResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}

// Function to extract CSRF token from JWT claims
pub fn extract_csrf_token_from_jwt(token: &str, secret: &[u8]) -> Option<String> {
    if let Ok(claims) = verify_jwt(token, secret) {
        Some(claims.csrf_token)
    } else {
        None
    }
}

// Function to validate the CSRF token from the JWT against the token provided in the request headers
pub fn validate_csrf_token(extracted_token: &str, header_csrf_token: &str) -> bool {
    extracted_token == header_csrf_token
}
