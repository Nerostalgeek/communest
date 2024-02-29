use lettre::{Message, SmtpTransport, Transport};
pub fn send_password_reset_email(
    to: &str,
    reset_token: &str,
) -> Result<(), lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from("no-reply@example.com".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Réinitialisation de votre mot de passe")
        .body(format!("Pour réinitialiser votre mot de passe, veuillez utiliser ce token : {}\nSi vous n'avez pas demandé à réinitialiser votre mot de passe, veuillez ignorer cet e-mail.", reset_token))
        .unwrap();

    let mailer = SmtpTransport::unencrypted_localhost();

    mailer.send(&email)?;

    Ok(())
}
