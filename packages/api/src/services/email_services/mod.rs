pub enum EmailType {
    Activation,
    Newsletter,
    PasswordReset,
}

// Cette structure représente le contexte nécessaire pour envoyer l'e-mail.
// Elle sera remplie avec les informations pertinentes pour chaque type d'e-mail.
pub struct EmailContext {
    pub recipient: String,
    pub token: String,
    // Ajoutez ici d'autres champs selon les besoins de vos e-mails.
}

#[derive(Debug)]
pub enum EmailServiceError {
    SendError(String),
}

impl fmt::Display for EmailServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailServiceError::SendError(err) => write!(f, "Email send error: {}", err),
        }
    }
}
// Déclaration des sous-modules.
pub mod activation;
pub mod password_reset;

use core::fmt;

// Importation des fonctions spécifiques de chaque sous-module.
use crate::SmtpConfig;
use activation::send_activation_email;
use actix_web::web;
use password_reset::send_password_reset_email;

// La fonction `send_email` qui utilise les types d'e-mails et le contexte pour envoyer l'e-mail approprié.
pub async fn send_email(
    smtp_config: web::Data<SmtpConfig>,
    email_type: EmailType,
    context: &EmailContext,
) -> Result<(), EmailServiceError> {
    match email_type {
        EmailType::Activation => {
            send_activation_email(&smtp_config, &context.recipient, &context.token)
                .map_err(|e| EmailServiceError::SendError(e.to_string()))
        }
        EmailType::Newsletter => {
            // Here you would call `send_newsletter_email` when it's implemented
            // For now, let's return a placeholder error
            Err(EmailServiceError::SendError(
                "Newsletter not implemented yet".to_string(),
            ))
        }
        EmailType::PasswordReset => send_password_reset_email(&context.recipient, &context.token)
            .map_err(|e| EmailServiceError::SendError(e.to_string())),
    }
}
