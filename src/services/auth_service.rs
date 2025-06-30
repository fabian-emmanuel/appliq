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

pub struct AuthService {
    pub user_repo: Arc<UserRepository>,
    pub token_repo: Arc<TokenRepository>,
    pub email_service: Arc<EmailService>,
}

const INVALID_CREDENTIALS: &str = "Invalid email or password. Please check and try again.";

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>, token_repo: Arc<TokenRepository>, email_service: Arc<EmailService>) -> Arc<Self> {
        Arc::new(Self { user_repo, token_repo, email_service })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<JwtToken, AppError> {
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        let user = self
            .user_repo
            .get_user_by_email(req.email.clone())
            .await
            .map_err(|e| {
                error!("Failed to find user by email {}: {:?}", req.email, e);
                AppError::BadRequest(String::from(INVALID_CREDENTIALS))
            })?;

        let is_password_valid = verify(&req.password, &user.password)
            .map_err(|e| {
                error!("Password verification failed for user_id {}: {:?}", user.id, e);
                AppError::BadRequest(String::from(INVALID_CREDENTIALS))
            })?;

        if !is_password_valid {
            return Err(AppError::BadRequest(String::from(INVALID_CREDENTIALS)));
        }

        Ok(create_jwt(&user.id, &user.role, req.remember_me))
    }

    pub async fn forgot_password(&self, req: ForgotPasswordRequest) -> Result<(), AppError> {
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        let user = match self.user_repo.get_user_by_email(req.email.clone()).await {
            Ok(user) => user,
            Err(_) => return Ok(())
        };

        if let Err(e) = self.token_repo.invalidate_existing_tokens_for_user(user.id).await {
            error!("Failed to invalidate existing tokens for user {}: {:?}", user.id, e);
            return Err(AppError::DatabaseError(e.to_string()));
        }

        let reset_token = Token::new(user.id);

        if let Err(e) = self.token_repo.save(reset_token.clone()).await {
            return Err(AppError::DatabaseError(e.to_string()));
        }


        // Clone data for the closure
        let email_service = self.email_service.clone();
        let full_name = format!("{} {}", user.first_name, user.last_name);
        let user_email = user.email.clone();
        let token_str = reset_token.token.clone();
        let expires_at = reset_token.expires_at;

        // Spawn a task to send the email without blocking the response
        tokio::spawn(async move {
            if let Err(e) = email_service.send_password_reset_email(&user_email, &full_name, &token_str, &expires_at).await {
                error!("Failed to send password reset email to {}: {:?}", user_email, e);
            }
        });

        Ok(())
    }

    pub async fn reset_password(&self, req: ResetPasswordRequest) -> Result<(), AppError> {
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        let token = self
            .token_repo
            .find_by_token(&req.token)
            .await
            .map_err(|e| {
                error!("Failed to find reset token: {:?}", e);
                AppError::BadRequest("Invalid or expired token".into())
            })?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired token".into()))?;

        if !token.is_valid() {
            return Err(AppError::BadRequest("Invalid or expired token".into()));
        }

        let user = self
            .user_repo
            .get_user_by_id(token.user_id)
            .await
            .map_err(|e| {
                error!("Failed to find user for token: {:?}", e);
                AppError::BadRequest("Invalid token".into())
            })?;

        let password_hash = hash(&req.password, DEFAULT_COST)
            .map_err(|e| AppError::AuthError(format!("Failed to hash password: {}", e)))?;

        self.user_repo
            .update_password(user.id, password_hash)
            .await
            .map_err(|e| {
                error!("Failed to update password for user {}: {:?}", user.id, e);
                AppError::DatabaseError(e.to_string())
            })?;

        self.token_repo
            .mark_as_used(token.id)
            .await
            .map_err(|e| {
                error!("Failed to mark token as used: {:?}", e);
                AppError::DatabaseError(e.to_string())
            })?;

        Ok(())
    }

}
