use crate::handlers::user_handlers;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(user_handlers::list_users))
            .route("", web::post().to(user_handlers::create_user)), // Other user routes...
    );
}
