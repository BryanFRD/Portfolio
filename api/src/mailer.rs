use std::time::{SystemTime, UNIX_EPOCH};

use lettre::message::Mailbox;
use lettre::message::header::{Header, HeaderName, HeaderValue};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use rand::Rng;

use crate::model::ContactRequest;

#[derive(Debug)]
pub enum MailError {
    InvalidAddress(lettre::address::AddressError),
    Build(lettre::error::Error),
    Send(lettre::transport::smtp::Error),
}

impl std::fmt::Display for MailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAddress(error) => write!(f, "invalid reply-to address: {error}"),
            Self::Build(error) => write!(f, "failed to build email: {error}"),
            Self::Send(error) => write!(f, "failed to send email: {error}"),
        }
    }
}

impl std::error::Error for MailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidAddress(error) => Some(error),
            Self::Build(error) => Some(error),
            Self::Send(error) => Some(error),
        }
    }
}

/// Signale aux destinataires que le message est produit par un automate et non
/// écrit à la main, ce qui évite les réponses automatiques en boucle. RFC 3834 :
/// `auto-generated` pour la notification, `auto-replied` pour l'accusé.
#[derive(Clone)]
struct AutoSubmitted(&'static str);

impl Header for AutoSubmitted {
    fn name() -> HeaderName {
        HeaderName::new_from_ascii_str("Auto-Submitted")
    }

    fn parse(_value: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self("auto-generated"))
    }

    fn display(&self) -> HeaderValue {
        HeaderValue::new(Self::name(), self.0.to_owned())
    }
}

pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Mailbox,
}

impl Mailer {
    pub fn from_env() -> Option<Self> {
        let host = std::env::var("SMTP_HOST").ok()?;
        let username = std::env::var("SMTP_USERNAME").ok()?;
        let password = std::env::var("SMTP_PASSWORD").ok()?;
        let from = std::env::var("CONTACT_FROM").ok()?;
        let to = std::env::var("CONTACT_TO").ok()?;

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
            .expect("invalid SMTP_HOST")
            .credentials(Credentials::new(username, password))
            .build();

        Some(Self {
            transport,
            from: from.parse().expect("invalid CONTACT_FROM"),
            to: to.parse().expect("invalid CONTACT_TO"),
        })
    }

    /// Identifiant sur le domaine expéditeur. Sans cet en-tête, le relais en
    /// fabrique un sur son propre hôte, et un `Message-ID` étranger au domaine
    /// du `From` est un signal négatif pour les filtres anti-spam.
    fn message_id(&self) -> String {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or_default();
        let noise: u64 = rand::rng().random();
        format!("<{stamp:x}.{noise:x}@{}>", self.from.email.domain())
    }

    pub async fn send_contact(&self, request: &ContactRequest) -> Result<(), MailError> {
        let reply_to = Mailbox::new(
            Some(request.name.clone()),
            request.email.parse().map_err(MailError::InvalidAddress)?,
        );
        // Le domaine relaie le message d'un tiers : l'annoncer dans le `From`
        // vaut mieux que de laisser croire à une usurpation. Même convention
        // que GitHub ou Discourse.
        let from = Mailbox::new(
            Some(format!("{} (via Portfolio)", request.name)),
            self.from.email.clone(),
        );
        let email = Message::builder()
            .from(from)
            .reply_to(reply_to)
            .to(self.to.clone())
            .message_id(Some(self.message_id()))
            .header(AutoSubmitted("auto-generated"))
            .subject(format!("[Portfolio] Message de {}", request.name))
            .body(format!(
                "De : {} <{}>\n\n{}",
                request.name, request.email, request.message
            ))
            .map_err(MailError::Build)?;

        self.transport.send(email).await.map_err(MailError::Send)?;
        Ok(())
    }

    /// Accusé de réception envoyé au visiteur.
    ///
    /// Le corps ne reprend délibérément pas son message : l'adresse de
    /// destination est fournie par un inconnu, donc tout contenu recopié ici
    /// permettrait de faire transiter un texte arbitraire vers une adresse
    /// arbitraire, signé par le domaine. L'accusé est identique pour tous.
    pub async fn send_acknowledgement(&self, request: &ContactRequest) -> Result<(), MailError> {
        let to = Mailbox::new(
            Some(request.name.clone()),
            request.email.parse().map_err(MailError::InvalidAddress)?,
        );
        let email = Message::builder()
            .from(self.from.clone())
            .to(to)
            .message_id(Some(self.message_id()))
            .header(AutoSubmitted("auto-replied"))
            .subject("Votre message a bien été reçu")
            .body(format!(
                "Bonjour {},\n\nMerci pour votre message, je l'ai bien reçu et je vous \
                 réponds dès que possible.\n\nInutile de répondre à cet email, il est \
                 envoyé automatiquement.\n\nBryan Ferrando\nhttps://bryan-ferrando.fr\n",
                request.name
            ))
            .map_err(MailError::Build)?;

        self.transport.send(email).await.map_err(MailError::Send)?;
        Ok(())
    }
}
