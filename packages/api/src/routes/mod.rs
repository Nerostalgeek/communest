use actix_web::web;

pub mod task_routes;
pub mod user_routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    user_routes::init_routes(cfg);
    // task_routes::init_routes(cfg);
    // Add more routes as needed
}
