use crate::errors::app_error::{AppError, extract_validation_errors};
use crate::models::auth::LoginRequest;
use crate::repositories::user_repository::UserRepository;
use crate::utils::jwt::{Token, create_jwt};
use bcrypt::verify;
use std::sync::Arc;
use tracing::error;
use validator::Validate;

pub struct AuthService {
    pub user_repo: Arc<UserRepository>,
}

const INVALID_CREDENTIALS: &str = "Invalid email or password. Please check and try again.";

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn login(&self, req: LoginRequest) -> Result<Token, AppError> {
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
            error!("Invalid password for user_id: {}", user.id);
            return Err(AppError::BadRequest(String::from(INVALID_CREDENTIALS)));
        }

        Ok(create_jwt(&user.id, &user.role, req.remember_me))
    }
}
