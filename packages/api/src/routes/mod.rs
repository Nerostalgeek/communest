// In src/routes/mod.rs

use actix_web::web;

// Bring each version's routing configuration into scope
pub mod v1;
// Future versions would be added here, e.g.:
// pub mod v2;

// This function can be used to configure all routes for the application.
pub fn init(cfg: &mut web::ServiceConfig) {
    // Initialize routes for version 1 of the API
    cfg.configure(v1::init_routes);

    // When version 2 of the API is developed, you'd add another configure call here:
    // cfg.configure(v2::init);
}
