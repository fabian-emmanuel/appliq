use crate::enums::roles::Role;
use crate::models::user::User;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
#[schema(description = "Payload for creating a new user.")]
pub struct UserRequest {
    #[serde(rename = "firstName")]
    #[schema(description = "User's first name.", example = "John")]
    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: String,

    #[serde(rename = "lastName")]
    #[schema(description = "User's last name.", example = "Doe")]
    #[validate(length(min = 1, message = "Last name cannot be empty"))]
    pub last_name: String,

    #[schema(description = "User's email address.", example = "user@example.com")]
    #[validate(email(message = "Email must be valid"), length(min = 6))]
    pub email: String,

    #[schema(description = "User's password. Must be at least 6 characters long.", example = "securepassword")]
    #[validate(length(min = 6, message = "Password must be more than 5 characters long"))]
    pub password: String,

    #[schema(description = "Role to assign to the new user. Defaults to 'User' if not provided.")]
    pub role: Option<Role>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(description = "Detailed information about a user.")]
pub struct UserInfo {
    #[schema(description = "Unique identifier of the user.", example = 1)]
    pub id: i64,
    
    #[serde(rename = "firstName")]
    #[schema(description = "User's first name.", example = "John")]
    pub first_name: String,
    
    #[serde(rename = "lastName")]
    #[schema(description = "User's last name.", example = "Doe")]
    pub last_name: String,
    
    #[schema(description = "User's email address.", example = "user@example.com")]
    pub email: String,
    
    #[schema(description = "Role assigned to the user.")]
    pub role: Role,
    
    #[serde(rename = "createdAt")]   
    #[schema(description = "Timestamp of when the user account was created.")]
    pub created_at: DateTime<Local>,

    #[serde(rename = "lastLoginAt")]
    #[schema(description = "Timestamp of the user's last login.")]
    pub last_login_at: Option<DateTime<Local>>,

    #[serde(rename = "isVerified")]
    #[schema(description = "Flag indicating if the user's email has been verified.", example = true)]
    pub is_verified: bool,
}

impl UserInfo {
    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            created_at: user.created_at.clone(),
            last_login_at: user.last_login_at.clone(),
            is_verified: user.is_verified.clone(),
        }
    }
}
