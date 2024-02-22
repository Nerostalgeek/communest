use crate::schema::users;
use chrono::NaiveDateTime;
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
    pub token_expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct LoginRequest {
    pub email: String,
    pub password_hash: String,
}
