use actix_web::{middleware::Logger, web, App, HttpServer};
use config::config::AppConfig;
use config::smtp;
use lettre::transport::smtp::client::SmtpConnection;

use std::io;

mod config;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use crate::config::db;
use crate::routes::v1;
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize database pool
    let pool = db::init_database_pool();
    // Initialize AppConfig
    let config = AppConfig::new();

    let server_bind_address = config.base_url.clone();
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1").configure(v1::init_routes), // Your routes are now scoped to `/api`
            )
    })
    .bind(server_bind_address)?
    .run()
    .await
}
