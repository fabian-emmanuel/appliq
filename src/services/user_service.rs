use crate::errors::app_error::{AppError, extract_validation_errors};
use crate::models::user::User;
use crate::payloads::user::{UserInfo, UserRequest};
use crate::repositories::user_repository::UserRepository;
use bcrypt::{DEFAULT_COST, hash};
use std::sync::Arc;
use tracing::error;
use validator::Validate;

/// # User Service
///
/// Provides business logic for user management, such as registration
/// and retrieval of user information. It interacts with the `UserRepository`
/// for database operations.
pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    /// Creates a new instance of `UserService`.
    ///
    /// # Parameters
    /// - `user_repo`: An `Arc` wrapped `UserRepository` for database interactions.
    ///
    /// # Returns
    /// An `Arc` wrapped `UserService` instance.
    pub fn new(user_repo: Arc<UserRepository>) -> Arc<Self> {
        Arc::new(Self { user_repo })
    }

    /// Registers a new user in the system.
    ///
    /// This function performs the following steps:
    /// 1. Validates the input `UserRequest` payload.
    /// 2. Checks if a user with the provided email already exists.
    /// 3. Hashes the user's password.
    /// 4. Creates a new `User` model instance.
    /// 5. Saves the new user to the database via the `UserRepository`.
    /// 6. Returns the information of the newly created user as `UserInfo`.
    ///
    /// # Parameters
    /// - `registration_data`: A `UserRequest` struct containing the new user's details
    ///   (first name, last name, email, password, optional role).
    ///
    /// # Returns
    /// - `Ok(UserInfo)`: Information about the successfully registered user.
    /// - `Err(AppError)`: An error if validation fails, the email already exists,
    ///   password hashing fails, or a database operation fails.
    ///
    /// # Errors
    /// - `AppError::ValidationError` if the `registration_data` is invalid.
    /// - `AppError::ResourceExists` if a user with the given email already exists.
    /// - `AppError::DatabaseError` if there's an issue checking email existence or saving the user.
    /// - `AppError::AuthError` if password hashing fails.
    pub async fn register_user(
        &self,
        registration_data: UserRequest,
    ) -> Result<UserInfo, AppError> {
        // Validate the incoming registration data.
        registration_data
            .validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        // Check if the email is already in use.
        match self
            .user_repo
            .exists_by_email(registration_data.email.clone())
            .await
        {
            Ok(true) => {
                error!("Attempt to register with existing email: {}", registration_data.email);
                return Err(AppError::ResourceExists("Email already in use.".into()));
            }
            Ok(false) => (), // Email doesn't exist, proceed with registration.
            Err(e) => {
                error!("Database error while checking email existence for {}: {:?}", registration_data.email, e);
                return Err(AppError::DatabaseError(e.to_string()));
            }
        }

        // Hash the user's password.
        let password_hash = hash(&registration_data.password, DEFAULT_COST)
            .map_err(|e| {
                error!("Failed to hash password for new user {}: {}", registration_data.email, e);
                AppError::AuthError(format!("Failed to hash password: {}", e))
            })?;

        // Create a new User model instance.
        let new_user = User::new(
            registration_data.first_name,
            registration_data.last_name,
            registration_data.email,
            password_hash,
            registration_data.role, // Role is optional and handled by User::new
        );

        // Save the new user to the database.
        self.user_repo
            .save(new_user)
            .await
            .map(|user| UserInfo::from_user(&user)) // Convert User model to UserInfo DTO
            .map_err(|e| {
                error!("Failed to save new user to database: {:?}", e);
                AppError::DatabaseError(e.to_string())
            })
    }

    /// Retrieves detailed information for a specific user by their ID.
    ///
    /// # Parameters
    /// - `user_id`: The unique identifier (`i64`) of the user to retrieve.
    ///
    /// # Returns
    /// - `Ok(UserInfo)`: A `UserInfo` struct containing the user's details if found.
    /// - `Err(AppError)`: An `AppError::ResourceNotFound` if no user with the given ID exists,
    ///   or `AppError::DatabaseError` if there's an issue querying the database.
    ///
    /// # Errors
    /// - `AppError::ResourceNotFound` if the user with the specified `user_id` is not found.
    /// - `AppError::DatabaseError` if a database query fails.
    pub async fn get_user_data(&self, user_id: i64) -> Result<UserInfo, AppError> {
        // Fetch the user by ID from the repository.
        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .await
            .map_err(|err| {
                error!("Failed to fetch user with ID {}: {}", user_id, err);
                // Distinguish between "not found" and other DB errors if repository allows,
                // otherwise, assume not found for simplicity here.
                AppError::ResourceNotFound(String::from("User not found."))
            })?;

        // Convert the User model to UserInfo DTO.
        // This mapping ensures that sensitive fields (like password hash) are not exposed.
        Ok(UserInfo {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
            is_verified: user.is_verified,
        })
    }
}
