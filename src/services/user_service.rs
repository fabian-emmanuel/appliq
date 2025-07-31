use crate::errors::app_error::{AppError, extract_validation_errors};
use crate::models::user::User;
use crate::payloads::user::{UpdateUser, UserInfo, UserRequest};
use crate::repositories::user_repository::UserRepository;
use bcrypt::{DEFAULT_COST, hash};
use std::sync::Arc;
use tracing::error;
use validator::Validate;

pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<UserRepository>) -> Arc<Self> {
        Arc::new(Self { user_repo })
    }

    pub async fn register_user(
        &self,
        registration_data: UserRequest,
    ) -> Result<UserInfo, AppError> {
        registration_data
            .validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        match self
            .user_repo
            .exists_by_email(registration_data.email.clone())
            .await
        {
            Ok(true) => return Err(AppError::ResourceExists("Email already in use.".into())),
            Ok(false) => (), // Email doesn't exist, continue with registration
            Err(e) => return Err(AppError::DatabaseError(e.to_string())),
        }

        let password_hash = hash(&registration_data.password, DEFAULT_COST)
            .map_err(|e| AppError::AuthError(format!("Failed to hash password: {}", e)))?;

        let new_user = User::new(
            registration_data.first_name,
            registration_data.last_name,
            registration_data.email,
            Option::from(registration_data.phone_number),
            password_hash,
            registration_data.role,
        );

        self.user_repo
            .save(new_user)
            .await
            .map(|user| UserInfo::from_user(&user))
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn get_user_data(&self, user_id: i64) -> Result<UserInfo, AppError> {
        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .await
            .map_err(|err| {
                error!("Failed to fetch user with ID {}: {}", user_id, err);
                AppError::ResourceNotFound(String::from("User not found."))
            })?;

        Ok(UserInfo::from_user(&user))
    }

    pub async fn update_user(&self, user_id: i64, update_data: UpdateUser) -> Result<UserInfo, AppError> {
        update_data.validate().map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        let mut current_user = self.user_repo.get_user_by_id(user_id).await.map_err(|err| {
            error!("Failed to fetch user with ID {}: {}", user_id, err);
            AppError::ResourceNotFound(String::from("User not found."))
        })?;

        if let Some(first_name) = update_data.first_name {
            current_user.first_name = first_name;
        }

        if let Some(last_name) = update_data.last_name {
            current_user.last_name = last_name;
        }

        if let Some(phone_number) = update_data.phone_number {
            current_user.phone_number = Some(phone_number);
        }

        if let Some(location) = update_data.location {
            current_user.location = Some(location);
        }

        if let Some(bio) = update_data.bio {
            current_user.bio = Some(bio);
        }

        self.user_repo.update_user(user_id, current_user).await
            .map(|user| UserInfo::from_user(&user))
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
