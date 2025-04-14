use crate::enums::roles::Role;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::option::Option;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, PartialEq)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,
    pub role: Role,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Local>>,

    #[serde(skip_serializing)]
    pub deleted: bool,

    pub is_verified: bool,
    pub last_login_at: Option<DateTime<Local>>,

    #[serde(skip_serializing)]
    pub failed_login_attempts: i32,
}

impl User {
    pub fn new(first_name: String, last_name:String, email:String, password:String, role: Option<Role>) -> Self {
        let now = Local::now();
        Self {
            id: 0,
            first_name,
            last_name,
            email,
            password,
            role: role.unwrap_or_else(|| Role::User),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            deleted: false,
            is_verified: false,
            last_login_at: None,
            failed_login_attempts: 0,
        }
    }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct UserRegistration {

    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "Last name cannot be empty"))]
    pub last_name: String,

    #[validate(email(message = "Email must be valid"), length(min = 6))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be more than 5 characters long"))]
    pub password: String,

    pub role: Option<Role>

}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: Role,
    pub created_at: DateTime<Local>,
    pub last_login_at: Option<DateTime<Local>>,
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
