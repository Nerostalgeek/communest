use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use std::io;

mod config;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use crate::config::config::AppConfig;
use crate::config::db;
use crate::routes::v1;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let pool = db::init_database_pool(); // Ensure your pool init function is async if it's awaited
    let config = AppConfig::new();

    let server_bind_address = config.base_url.clone();

    // Ensure you have a secret key for CSRF token generation

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
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1").configure(v1::init_routes))
    })
    .bind(server_bind_address)?
    .run()
    .await
}
