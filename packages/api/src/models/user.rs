use crate::schema::users;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub is_activated: bool,
    pub password_reset_token: Option<Uuid>,
    pub password_reset_expires_at: Option<DateTime<Utc>>,
    pub account_activation_token: Option<Uuid>,
    pub account_activation_token_expires_at: Option<DateTime<Utc>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub account_activation_token: Uuid,
    pub account_activation_token_expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}
#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}
#[derive(Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ValidateResetPasswordRequest {
    pub token: Uuid,
    pub new_password: String, // Ensure this is validated for security (e.g., minimum length)
}

#[derive(Deserialize)]
pub struct ActivateAccountRequest {
    pub token: Uuid,
}

#[derive(Deserialize)]
pub struct TokenRefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    // Optionally include other fields, such as:
    // pub token_type: String,
    // pub expires_in: usize,
}
