use crate::configs::routes::{RESET_PASSWORD_FE};
use crate::errors::app_error::AppError;
use crate::utils::date_util::format_relative_time;
use chrono::{DateTime, Local};
use lettre::message::{Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{SmtpTransport};
use std::env::var;
use std::sync::Arc;
use std::time::Duration;
use tera::{Context, Tera};
use tracing::{error};
use crate::utils::email_util::send_email;

pub struct EmailService {
    transport: SmtpTransport,
    from_email: Mailbox,
    app_url: String,
    templates: Tera,
}

impl EmailService {
    pub fn new() -> Arc<Self> {
        let smtp_host = var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse::<u16>()
            .expect("SMTP_PORT must be a valid number");
        let smtp_user = var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_password = var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let app_url = var("APP_URL").expect("APP_URL must be set");
        let from_email_str = var("FROM_EMAIL").expect("FROM_EMAIL must be set");

        let from_email: Mailbox = format!("AppliQ <{}>", from_email_str)
            .parse()
            .expect("FROM_EMAIL must be a valid email format");

        let creds = Credentials::new(smtp_user, smtp_password);

        let tls_parameters =
            TlsParameters::new(smtp_host.clone()).expect("Failed to configure TLS parameters");

        let transport = SmtpTransport::relay(&smtp_host)
            .expect("Failed to set up SMTP relay")
            .port(smtp_port)
            .credentials(creds)
            .timeout(Some(Duration::from_secs(5)))
            .tls(lettre::transport::smtp::client::Tls::Required(
                tls_parameters,
            ))
            .build();

        let templates =
            Tera::new("./resources/templates/emails/*").expect("Failed to initialize templates");

        Arc::new(Self {
            transport,
            from_email,
            app_url,
            templates,
        })
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        user_name: &str,
        token: &str,
        expires_at: &DateTime<Local>,
    ) -> Result<(), AppError> {

        let reset_link = format!("{}{}?token={}", self.app_url, RESET_PASSWORD_FE, token);
        let expires_formatted = format_relative_time(expires_at);

        // Build context for the email template
        let mut context = Context::new();
        context.insert("user_name", user_name);
        context.insert("reset_link", &reset_link);
        context.insert("expires_in", &expires_formatted);

        // Render email content
        let html_body = self
            .templates
            .render("password_reset.html", &context)
            .map_err(|e| {
                error!("Failed to render HTML template: {}", e);
                AppError::EmailError("Failed to render HTML template".to_string())
            })?;

        // Validate the recipient email
        let to_email: Mailbox = to_email.parse().map_err(|e| {
            error!("Invalid recipient email format: {}", e);
            AppError::EmailError("Invalid recipient email format".to_string())
        })?;

        send_email(
            &self.transport,
            &self.from_email,
            &to_email,
            "AppliQ Password Reset",
            html_body,
        )
    }
}
