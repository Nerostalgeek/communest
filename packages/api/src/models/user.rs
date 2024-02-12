use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    // Other fields...
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    // Other fields...
}

pub fn create_user<'a>(conn: &PgConnection, new_user: NewUser<'a>) -> QueryResult<User> {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}
