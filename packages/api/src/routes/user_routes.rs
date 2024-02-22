use crate::handlers::user_handlers::register_user;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // .route("", web::get().to(user_handlers::list_users))
            .route("", web::post().to(register_user)), // Other user routes...
                                                       // .route("/login", web::post().to(login)),
    );
}
