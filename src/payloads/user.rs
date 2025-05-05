use crate::enums::roles::Role;
use crate::models::user::User;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct UserRequest {
    #[serde(rename = "firstName")]
    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: String,

    #[serde(rename = "lastName")]
    #[validate(length(min = 1, message = "Last name cannot be empty"))]
    pub last_name: String,

    #[validate(email(message = "Email must be valid"), length(min = 6))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be more than 5 characters long"))]
    pub password: String,

    pub role: Option<Role>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    pub id: i64,
    
    #[serde(rename = "firstName")]
    pub first_name: String,
    
    #[serde(rename = "lastName")]
    pub last_name: String,
    
    pub email: String,
    
    pub role: Role,
    
    #[serde(rename = "createdAt")]   
    pub created_at: DateTime<Local>,

    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<DateTime<Local>>,

    #[serde(rename = "isVerified")]
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
