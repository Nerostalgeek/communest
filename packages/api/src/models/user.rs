use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub last_name: String,
    pub first_name: String,
    pub phone_number: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'req> {
    pub username: &'req str,
    pub email: &'req str,
    pub last_name: &'req str,
    pub first_name: &'req str,
    pub phone_number: &'req str,
    pub password_hash: &'req str,
}

pub fn create_user<'req>(conn: &PgConnection, new_user: NewUser<'req>) -> QueryResult<User> {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}
