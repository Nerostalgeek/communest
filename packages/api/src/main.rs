use actix_web::{web, App, HttpServer};
use config::config::AppConfig;
use std::io;

mod config;
mod handlers;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use crate::config::db;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize database pool
    let pool = db::init_database_pool();
    // Initialize AppConfig
    let config = AppConfig::new();
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
