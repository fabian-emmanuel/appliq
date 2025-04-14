use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::enums::application::Status;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ApplicationFilter {
    pub search: Option<String>,
    pub status: Option<Status>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}

