use crate::errors::app_error::AppError;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, SinglePart};
use lettre::{Message, Transport};
use std::env::var;
use tracing::{error, info};

pub fn send_email<T>(
    transport: &T,
    from_email: &Mailbox,
    to_email: &Mailbox,
    subject: &str,
    html_body: String,
) -> Result<(), AppError>
where
    T: Transport,
    <T as Transport>::Error: std::fmt::Display,
    <T as Transport>::Ok: std::fmt::Debug,
{
    let email = Message::builder()
        .from(from_email.clone())
        .to(to_email.clone())
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_body),
        )
        .map_err(|e| {
            error!("Failed to build email message: {}", e);
            AppError::EmailError("Failed to build email message".to_string())
        })?;

    match transport.send(&email) {
        Ok(response) => {
            info!("Email sent successfully. SMTP response: {:?}", response);
            Ok(())
        }
        Err(e) => {
            error!(
                "Failed to send email. SMTP details: host={}, timeout=5s, error={}",
                var("SMTP_HOST").unwrap_or_default(),
                e
            );
            Err(AppError::EmailError("Failed to send email".to_string()))
        }
    }
}
