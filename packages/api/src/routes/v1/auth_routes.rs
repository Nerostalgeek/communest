use crate::handlers::auth_handlers::{login_user, register_user};
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login_user))
            .route("/register", web::post().to(register_user)),
    );
}
