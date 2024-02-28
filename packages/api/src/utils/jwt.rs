use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: &str, secret: &[u8]) -> Result<String> {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn verify_jwt(token: &str, secret: &[u8]) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
