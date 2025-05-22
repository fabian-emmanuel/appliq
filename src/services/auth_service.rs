use crate::errors::app_error::{AppError, extract_validation_errors};
use crate::payloads::auth::{ForgotPasswordRequest, LoginRequest, ResetPasswordRequest};
use crate::repositories::user_repository::UserRepository;
use crate::utils::jwt::{JwtToken, create_jwt};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use tracing::error;
use validator::Validate;
use crate::models::token::Token;
use crate::repositories::token_repository::TokenRepository;
use crate::services::email_service::EmailService;

/// # Authentication Service
///
/// Handles user authentication, including login, password reset requests,
/// and password updates using reset tokens. It coordinates interactions between
/// `UserRepository`, `TokenRepository`, and `EmailService`.
pub struct AuthService {
    /// Shared reference to the user repository for database access.
    pub user_repo: Arc<UserRepository>,
    /// Shared reference to the token repository for managing password reset tokens.
    pub token_repo: Arc<TokenRepository>,
    /// Shared reference to the email service for sending notifications.
    pub email_service: Arc<EmailService>,
}

/// Constant error message for invalid login attempts.
const INVALID_CREDENTIALS: &str = "Invalid email or password. Please check and try again.";

impl AuthService {
    /// Creates a new instance of `AuthService`.
    ///
    /// # Parameters
    /// - `user_repo`: An `Arc` wrapped `UserRepository`.
    /// - `token_repo`: An `Arc` wrapped `TokenRepository`.
    /// - `email_service`: An `Arc` wrapped `EmailService`.
    ///
    /// # Returns
    /// An `Arc` wrapped `AuthService` instance.
    pub fn new(user_repo: Arc<UserRepository>, token_repo: Arc<TokenRepository>, email_service: Arc<EmailService>) -> Arc<Self> {
        Arc::new(Self { user_repo, token_repo, email_service })
    }

    /// Authenticates a user based on email and password.
    ///
    /// Validates the request, retrieves the user by email, verifies the password,
    /// and generates a JWT if credentials are valid.
    ///
    /// # Parameters
    /// - `req`: The `LoginRequest` payload containing email and password.
    ///
    /// # Returns
    /// - `Ok(JwtToken)`: A JWT token if authentication is successful.
    /// - `Err(AppError)`: An error if validation fails, user is not found,
    ///   password verification fails, or JWT creation fails.
    ///
    /// # Errors
    /// - `AppError::ValidationError` if the request payload is invalid.
    /// - `AppError::BadRequest` for invalid email/password or other verification issues.
    pub async fn login(&self, req: LoginRequest) -> Result<JwtToken, AppError> {
        // Validate the incoming request.
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        // Retrieve the user by email.
        let user = self
            .user_repo
            .get_user_by_email(req.email.clone())
            .await
            .map_err(|e| {
                error!("Failed to find user by email {}: {:?}", req.email, e);
                AppError::BadRequest(String::from(INVALID_CREDENTIALS))
            })?;

        // Verify the provided password against the stored hash.
        let is_password_valid = verify(&req.password, &user.password)
            .map_err(|e| {
                // Log the error but return a generic message to the user.
                error!("Password verification failed for user_id {}: {:?}", user.id, e);
                AppError::BadRequest(String::from(INVALID_CREDENTIALS))
            })?;

        if !is_password_valid {
            error!("Invalid password for user_id: {}", user.id);
            return Err(AppError::BadRequest(String::from(INVALID_CREDENTIALS)));
        }

        // Create and return a JWT for the authenticated user.
        Ok(create_jwt(&user.id, &user.role, req.remember_me))
    }

    /// Initiates the password reset process for a user.
    ///
    /// Validates the request, retrieves the user by email, invalidates any existing
    /// reset tokens for the user, creates a new reset token, saves it, and then
    /// asynchronously sends a password reset email to the user.
    ///
    /// If the user is not found, the function returns `Ok(())` to avoid disclosing
    /// whether an email address is registered.
    ///
    /// # Parameters
    /// - `req`: The `ForgotPasswordRequest` payload containing the user's email.
    ///
    /// # Returns
    /// - `Ok(())`: Indicates the process was initiated (or appeared to be, in case of unknown email).
    /// - `Err(AppError)`: An error if validation fails or a database/email operation fails.
    ///
    /// # Errors
    /// - `AppError::ValidationError` if the request payload is invalid.
    /// - `AppError::DatabaseError` if database operations fail.
    /// - Email sending errors are logged but do not cause the function to return an error.
    pub async fn forgot_password(&self, req: ForgotPasswordRequest) -> Result<(), AppError> {
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        // Attempt to retrieve the user. If not found, return Ok to prevent email enumeration.
        let user = match self.user_repo.get_user_by_email(req.email.clone()).await {
            Ok(user) => user,
            Err(_) => {
                // Log that an attempt was made for a non-existent email, if desired for monitoring.
                // info!("Password reset requested for non-existent email: {}", req.email);
                return Ok(()); // Do not disclose if email exists.
            }
        };

        // Invalidate any previous tokens for this user to ensure only the latest one is valid.
        if let Err(e) = self.token_repo.invalidate_existing_tokens_for_user(user.id).await {
            error!("Failed to invalidate existing tokens for user {}: {:?}", user.id, e);
            return Err(AppError::DatabaseError(e.to_string()));
        }

        // Create a new password reset token.
        let reset_token = Token::new(user.id);

        // Save the new token to the database.
        if let Err(e) = self.token_repo.save(reset_token.clone()).await {
            error!("Failed to save new reset token for user {}: {:?}", user.id, e);
            return Err(AppError::DatabaseError(e.to_string()));
        }

        // Prepare data for email sending.
        let email_service = self.email_service.clone();
        let full_name = format!("{} {}", user.first_name, user.last_name);
        let user_email = user.email.clone(); // User's actual email.
        let token_str = reset_token.token.clone(); // The token string.
        let expires_at = reset_token.expires_at; // Token expiry timestamp.

        // Spawn a non-blocking task to send the password reset email.
        // This allows the API to respond quickly without waiting for email sending.
        tokio::spawn(async move {
            if let Err(e) = email_service.send_password_reset_email(&user_email, &full_name, &token_str, &expires_at).await {
                error!("Failed to send password reset email to {}: {:?}", user_email, e);
            }
        });

        Ok(())
    }

    /// Resets a user's password using a provided reset token.
    ///
    /// Validates the request, finds the token, checks its validity, retrieves the user,
    /// hashes the new password, updates the user's password in the database,
    /// and marks the token as used.
    ///
    /// # Parameters
    /// - `req`: The `ResetPasswordRequest` payload containing the new password,
    ///   confirmation, and the reset token.
    ///
    /// # Returns
    /// - `Ok(())`: Indicates the password was successfully reset.
    /// - `Err(AppError)`: An error if validation fails, token is invalid/expired,
    ///   user is not found, or a database operation fails.
    ///
    /// # Errors
    /// - `AppError::ValidationError` if the request payload is invalid.
    /// - `AppError::BadRequest` for invalid or expired tokens.
    /// - `AppError::AuthError` if password hashing fails.
    /// - `AppError::DatabaseError` if database operations fail.
    pub async fn reset_password(&self, req: ResetPasswordRequest) -> Result<(), AppError> {
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        // Find the token in the database.
        let token = self
            .token_repo
            .find_by_token(&req.token)
            .await
            .map_err(|e| {
                error!("Database error while finding reset token: {:?}", e);
                AppError::BadRequest("Invalid or expired token.".into()) // Generic message
            })?
            .ok_or_else(|| {
                error!("Reset token not found: {}", req.token);
                AppError::BadRequest("Invalid or expired token.".into())
            })?;

        // Check if the token is still valid (not expired and not used).
        if !token.is_valid() {
            error!("Attempt to use invalid or expired token ID: {}", token.id);
            return Err(AppError::BadRequest("Invalid or expired token.".into()));
        }

        // Retrieve the user associated with the token.
        let user = self
            .user_repo
            .get_user_by_id(token.user_id)
            .await
            .map_err(|e| {
                error!("Failed to find user for token ID {}: {:?}", token.id, e);
                // This might indicate a data integrity issue or a very old token.
                AppError::BadRequest("Invalid token: User not found.".into())
            })?;

        // Hash the new password.
        let password_hash = hash(&req.password, DEFAULT_COST)
            .map_err(|e| {
                error!("Failed to hash new password for user {}: {}", user.id, e);
                AppError::AuthError(format!("Failed to process new password: {}", e))
            })?;

        // Update the user's password in the database.
        self.user_repo
            .update_password(user.id, password_hash)
            .await
            .map_err(|e| {
                error!("Failed to update password for user {}: {:?}", user.id, e);
                AppError::DatabaseError(e.to_string())
            })?;

        // Mark the token as used to prevent reuse.
        self.token_repo
            .mark_as_used(token.id)
            .await
            .map_err(|e| {
                error!("Failed to mark token ID {} as used: {:?}", token.id, e);
                // This is critical; if this fails, the token might be reused.
                // Depending on policy, might warrant a more severe error or manual intervention alert.
                AppError::DatabaseError(e.to_string())
            })?;

        Ok(())
    }

}
