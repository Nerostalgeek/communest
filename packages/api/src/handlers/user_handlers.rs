use crate::config::db::DbPool;
use crate::models::user::{create_user, NewUser};
use actix_web::{web, HttpResponse, Responder};
use diesel::PgConnection;

pub async fn register_user<'req>(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser<'req>>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    match create_user(&conn, new_user.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
