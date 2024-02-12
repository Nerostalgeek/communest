use actix_web::{web, App, HttpServer};
use std::io;

mod config;
mod handlers;
mod models;
mod routes;

use crate::config::db;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize database pool
    let pool = db::init_database_pool("DATABASE_URL");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // Pass database pool to handlers
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
