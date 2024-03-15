use crate::handlers::user_handlers::{delete_user, get_user, get_users, update_user};
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))
            .route("/{user_id}", web::get().to(get_user))
            .route("/{user_id}", web::put().to(update_user))
            .route("/{user_id}", web::delete().to(delete_user)),
    );
}
