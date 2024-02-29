// Enums pour représenter les différents types d'e-mails que l'on peut envoyer.
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

// Une erreur personnalisée pour les opérations de service d'e-mail.
#[derive(Debug)]
pub enum EmailServiceError {
    SendError(String),
    // Vous pouvez ajouter d'autres types d'erreurs ici si nécessaire.
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
use activation::send_activation_email;
use password_reset::send_password_reset_email;

// La fonction `send_email` qui utilise les types d'e-mails et le contexte pour envoyer l'e-mail approprié.
pub async fn send_email(
    email_type: EmailType,
    context: &EmailContext,
) -> Result<(), EmailServiceError> {
    match email_type {
        EmailType::Activation => send_activation_email(&context.recipient, &context.token)
            .map_err(|e| EmailServiceError::SendError(e.to_string())),
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
