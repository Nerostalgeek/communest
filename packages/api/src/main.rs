use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::config::AppConfig;
use std::{env, io};

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
        let cors = Cors::default()
            .allowed_origin(&config.web_base_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1").configure(v1::init_routes))
    })
    .bind(server_bind_address)?
    .run()
    .await
}
