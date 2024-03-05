use crate::config::smtp::SmtpConfig;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use native_tls::TlsConnector;

pub fn send_activation_email(
    config: &SmtpConfig,
    to: &str,
    token: &str,
) -> Result<(), lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from(Mailbox::new(None, "no-reply@example.com".parse().unwrap()))
        .to(Mailbox::new(None, to.parse().unwrap()))
        .subject("Activation de votre compte")
        .body(format!(
            "Veuillez activer votre compte en utilisant ce token: {}",
            token
        ))?;

    let creds = Credentials::new(config.username.clone(), config.password.clone());

    let tls_connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mailer = SmtpTransport::relay(&config.server)?
        .port(config.port)
        .credentials(creds)
        .tls(TransportBuilder::tls_with_parameters(tls_connector))
        .build();

    mailer.send(&email)?;
    Ok(())
}
