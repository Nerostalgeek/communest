use actix_web::{middleware::Logger, web, App, HttpServer};
use config::config::AppConfig;
use config::smtp::SmtpConfig;

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

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize database pool
    let pool = db::init_database_pool();
    // Initialize AppConfig
    let config = AppConfig::new();
    let smtp = SmtpConfig::new();
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .app_data(web::Data::new(smtp.clone()))
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
