// Define the initialization for version 1 routes.

use actix_web::web;

pub mod auth_routes;
pub mod task_routes;
pub mod user_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    user_routes::init_routes(cfg);
    auth_routes::init_routes(cfg);
    // task_routes::init_routes(cfg);
    // ... initialization of other V1 routes ...
}
