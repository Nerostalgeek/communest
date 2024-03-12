use std::env;
use std::sync::Arc;

use actix_web::Error;
use sendgrid::{Destination, Mail, SGClient};

pub async fn send_password_reset_email(
    sg_client: Arc<SGClient>,
    to: &str,
    reset_token: &str,
) -> Result<(), Error> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "127.0.0.1".to_string());

    let from = env::var("EMAIL_SENDER").expect("SG EMAIL_SENDER must be set in the environment.");

    let subject = "Réinitialisation de votre mot de passe";
    let content_value = format!(
        "Pour réinitialiser votre mot de passe, veuillez utiliser ce lien : 
    {}/reset-password?token={} 
    \nSi vous n'avez pas demandé à réinitialiser votre mot de passe, veuillez ignorer cet e-mail.",
        base_url, reset_token
    );

    let to_destination = Destination {
        address: to,
        name: "", // Vous pouvez spécifier un nom ici si vous le souhaitez
    };

    let mail = Mail::new()
        .add_from(&from)
        .add_subject(subject)
        .add_text(&content_value)
        .add_to(to_destination);

    sg_client.send(mail).await.map_err(|e| {
        Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Sendgrid error: {}", e),
        ))
    })?;
    Ok(())
}
