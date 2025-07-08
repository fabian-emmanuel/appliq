use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone, PartialEq)]
pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expires_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub used: bool,
}

impl Token {
    pub fn new(user_id: i64) -> Self {
        let token = Uuid::new_v4().to_string();
        let now = Local::now();
        let expires_at = now + Duration::from_secs(660); // 10 Min expiration

        Self {
            id: 0, // Will be set by the database
            user_id,
            token,
            expires_at,
            created_at: now,
            used: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.used && self.expires_at > Local::now()
    }
}
