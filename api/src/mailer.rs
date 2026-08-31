use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

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

    pub async fn send_contact(&self, request: &ContactRequest) -> Result<(), MailError> {
        let reply_to = Mailbox::new(
            Some(request.name.clone()),
            request.email.parse().map_err(MailError::InvalidAddress)?,
        );
        let email = Message::builder()
            .from(self.from.clone())
            .reply_to(reply_to)
            .to(self.to.clone())
            .subject(format!("[Portfolio] Message de {}", request.name))
            .body(format!(
                "De : {} <{}>\n\n{}",
                request.name, request.email, request.message
            ))
            .map_err(MailError::Build)?;

        self.transport.send(email).await.map_err(MailError::Send)?;
        Ok(())
    }
}
