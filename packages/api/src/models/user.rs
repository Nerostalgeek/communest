use crate::schema::users;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid, // Adjusted for UUID
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub is_verified: bool,
    pub verification_token: Option<Uuid>,
    pub token_expires_at: Option<DateTime<Utc>>,
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
    pub verification_token: Uuid, // Included in the struct for database insertion
    pub token_expires_at: DateTime<Utc>, // Included in the struct for database insertion
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String, // This is the raw password, which will be hashed in the service layer
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct LoginRequest {
    pub email: String,
    pub password_hash: String,
}
