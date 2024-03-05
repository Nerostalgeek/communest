use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
}

impl SmtpConfig {
    pub fn new() -> Self {
        // Exemple d'utilisation de variables d'environnement
        dotenv::dotenv().ok(); // Chargez les variables d'environnement à partir du fichier .env

        let server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
        let port: u16 = env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse()
            .expect("SMTP_PORT must be a valid port number");
        let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let use_tls = env::var("SMTP_USE_TLS").unwrap_or_else(|_| "true".to_string()) == "true";
        SmtpConfig {
            server,
            port,
            username,
            password,
            use_tls,
        }
    }
}
