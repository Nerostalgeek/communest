use lettre::{Message, SmtpTransport, Transport};

pub fn send_activation_email(to: &str, token: &str) -> Result<(), lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from("no-reply@example.com".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Activation de votre compte")
        .body(format!(
            "Veuillez activer votre compte en utilisant ce token: {}",
            token
        ))
        .unwrap();

    // Pour cet exemple, utilisons un transport SMTP non chiffré vers localhost.
    // Dans un scénario réel, vous devrez configurer cela pour votre serveur SMTP avec une sécurité appropriée.
    let mailer = SmtpTransport::unencrypted_localhost();

    mailer.send(&email)?;

    Ok(())
}
