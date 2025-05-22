use crate::configs::routes::RESET_PASSWORD;
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
use tracing::{error, info};
use crate::utils::email_util::send_email;

/// # Email Service
///
/// Handles the sending of emails, such as password reset notifications.
/// It configures an SMTP transport using environment variables and uses Tera templates
/// for rendering email content.
pub struct EmailService {
    /// The SMTP transport used to send emails.
    transport: SmtpTransport,
    /// The sender's email address, parsed into a `Mailbox`.
    from_email: Mailbox,
    /// The base URL of the application, used for constructing links in emails.
    app_url: String,
    /// Tera templating engine instance for rendering email templates.
    templates: Tera,
}

impl EmailService {
    /// Creates a new instance of `EmailService`.
    ///
    /// Initializes the SMTP transport and Tera templating engine based on
    /// environment variables:
    /// - `SMTP_HOST`, `SMTP_PORT`, `SMTP_USER`, `SMTP_PASSWORD`: For SMTP configuration.
    /// - `APP_URL`: Base URL for the application.
    /// - `FROM_EMAIL`: Sender's email address.
    ///
    /// It also loads email templates from `./resources/templates/emails/*`.
    ///
    /// # Returns
    /// An `Arc` wrapped `EmailService` instance.
    ///
    /// # Panics
    /// - Panics if any of the required environment variables are not set.
    /// - Panics if `SMTP_PORT` is not a valid number.
    /// - Panics if `FROM_EMAIL` is not a valid email format.
    /// - Panics if TLS parameters for SMTP cannot be configured.
    /// - Panics if the SMTP relay cannot be set up.
    /// - Panics if email templates cannot be loaded or parsed.
    pub fn new() -> Arc<Self> {
        // Load SMTP and application configuration from environment variables.
        let smtp_host = var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse::<u16>()
            .expect("SMTP_PORT must be a valid number");
        let smtp_user = var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_password = var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let app_url = var("APP_URL").expect("APP_URL must be set");
        let from_email_str = var("FROM_EMAIL").expect("FROM_EMAIL must be set");

        // Parse the sender's email address.
        let from_email: Mailbox = format!("AppliQ <{}>", from_email_str)
            .parse()
            .expect("FROM_EMAIL string must be a valid email format (e.g., 'no-reply@example.com')");

        // Create SMTP credentials.
        let creds = Credentials::new(smtp_user, smtp_password);

        // Configure TLS for the SMTP connection.
        let tls_parameters =
            TlsParameters::new(smtp_host.clone()).expect("Failed to configure TLS parameters for SMTP host");

        // Build the SMTP transport.
        // Uses relay, port, credentials, timeout, and requires TLS.
        let transport = SmtpTransport::relay(&smtp_host)
            .expect("Failed to set up SMTP relay")
            .port(smtp_port)
            .credentials(creds)
            .timeout(Some(Duration::from_secs(5))) // Set a 5-second timeout for SMTP operations.
            .tls(lettre::transport::smtp::client::Tls::Required(
                tls_parameters,
            ))
            .build();

        // Initialize the Tera templating engine.
        // Loads all HTML templates from the specified directory.
        let templates =
            Tera::new("./resources/templates/emails/*").expect("Failed to initialize Tera templates from ./resources/templates/emails/");

        Arc::new(Self {
            transport,
            from_email,
            app_url,
            templates,
        })
    }

    /// Sends a password reset email to the specified user.
    ///
    /// Renders the `password_reset.html` template with the user's name,
    /// a password reset link containing the token, and the token's expiry time.
    ///
    /// # Parameters
    /// - `to_email`: The recipient's email address.
    /// - `user_name`: The name of the user receiving the email.
    /// - `token`: The password reset token.
    /// - `expires_at`: The `DateTime<Local>` when the token will expire.
    ///
    /// # Returns
    /// - `Ok(())` if the email sending process was initiated successfully.
    /// - `Err(AppError)` if there's an error rendering the template, parsing the recipient email,
    ///   or if the underlying `send_email` utility function returns an error.
    ///
    /// # Errors
    /// - `AppError::EmailError` if template rendering fails or `to_email` is invalid.
    ///   Errors from the actual email sending are propagated from `utils::email_util::send_email`.
    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        user_name: &str,
        token: &str,
        expires_at: &DateTime<Local>,
    ) -> Result<(), AppError> {
        info!("Preparing to send password reset email to {}", to_email);

        // Construct the password reset link.
        let reset_link = format!("{}{}?token={}", self.app_url, RESET_PASSWORD, token);
        // Format the expiry time into a user-friendly relative string.
        let expires_formatted = format_relative_time(expires_at);

        // Create context for the email template.
        let mut context = Context::new();
        context.insert("user_name", user_name);
        context.insert("reset_link", &reset_link);
        context.insert("expires_in", &expires_formatted);

        // Render the HTML body of the email using the "password_reset.html" template.
        let html_body = self
            .templates
            .render("password_reset.html", &context)
            .map_err(|e| {
                error!("Failed to render HTML template 'password_reset.html': {}", e);
                AppError::EmailError(format!("Failed to render email template: {}", e))
            })?;

        // Validate and parse the recipient's email address.
        let to_mailbox: Mailbox = to_email.parse().map_err(|e| {
            error!("Invalid recipient email format for '{}': {}", to_email, e);
            AppError::EmailError(format!("Invalid recipient email format: {}", e))
        })?;

        // Use the utility function to send the email.
        send_email(
            &self.transport,
            &self.from_email,
            &to_mailbox, // Use the parsed mailbox.
            "AppliQ Password Reset",
            html_body,
        )
    }
}
