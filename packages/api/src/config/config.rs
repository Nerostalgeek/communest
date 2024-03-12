use sendgrid::SGClient;
use std::{env, sync::Arc};

use crate::config::smtp::create_sendgrid_client;
#[derive(Clone)]
pub struct AppConfig {
    pub base_url: String,
    pub jwt_secret: String,
    pub smtp_client: Arc<SGClient>,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        env_logger::init();

        let sendgrid_api_key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");

        let smtp_client = Arc::new(create_sendgrid_client(&sendgrid_api_key));
        let base_url = env::var("BASE_URL").expect("BASE_URL must be set in the environment.");
        let jwt_secret =
            env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment.");

        AppConfig {
            base_url,
            jwt_secret,
            smtp_client,
        }
    }
}
