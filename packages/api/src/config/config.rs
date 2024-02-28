use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let jwt_secret =
            env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment.");

        AppConfig { jwt_secret }
    }
}
