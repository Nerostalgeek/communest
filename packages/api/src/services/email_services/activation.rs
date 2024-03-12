use actix_web::Error;
use sendgrid::{Destination, Mail, SGClient};
use std::{env, sync::Arc};

pub async fn send_activation_email(
    sg_client: Arc<SGClient>,
    to: &str,
    token: &str,
) -> Result<(), Error> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let from = env::var("EMAIL_SENDER").expect("SG EMAIL_SENDER must be set in the environment.");
    let subject = "Activation de votre compte";
    let content_value = format!(
        "Veuillez activer votre compte en utilisant ce lien: {}/activate?token={}",
        base_url, token
    );

    let to_destination = Destination {
        address: to,
        name: "", // Vous pouvez spécifier un nom ici si vous le souhaitez
    };

    let mail = Mail::new()
        .add_from(&from)
        .add_subject(&subject)
        .add_text(&content_value)
        .add_to(to_destination); // Ajoutez Destination ici

    sg_client.send(mail).await.map_err(|e| {
        Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Sendgrid error: {}", e),
        ))
    })?;

    Ok(())
}
