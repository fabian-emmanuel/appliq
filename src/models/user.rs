use crate::enums::roles::Role;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::option::Option;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, PartialEq)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,

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
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        phone_number: Option<String>,
        password: String,
        role: Option<Role>,
    ) -> Self {
        let now = Local::now();
        Self {
            id: 0,
            first_name,
            last_name,
            email,
            password,
            phone_number,
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
