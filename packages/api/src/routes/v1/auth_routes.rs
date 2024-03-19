use crate::handlers::auth_handlers::{
    activate_account, confirm_password_reset, login_user, refresh_jtw_token, register_user,
    request_password_reset,
};
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login_user))
            .route("/register", web::post().to(register_user))
            .route(
                "/password-reset/request",
                web::post().to(request_password_reset),
            )
            .route(
                "/password-reset/confirm",
                web::post().to(confirm_password_reset),
            )
            .route("/activate/{token}", web::get().to(activate_account))
            .route("/refresh-token", web::post().to(refresh_jtw_token)),
    );
}
