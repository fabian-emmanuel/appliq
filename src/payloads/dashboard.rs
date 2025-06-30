use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::enums::application::Status;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DashboardCount {
    
    #[serde(rename = "totalApplications")]
    pub total_applications: i64,
    
    pub interviews: i64,
    
    pub tests: i64,
    
    #[serde(rename = "offersAwarded")]
    pub offers_awarded: i64,
    
    pub withdrawn: i64,
    
    pub rejected: i64,
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct SuccessRate {
    
    pub percentage: String,
    
    pub message: String,
}


#[derive(Serialize, Deserialize)]
pub struct AverageResponseTime {
    
    pub average: String,

    #[serde(rename = "fasterMessage")]
    pub faster_message: String,

    #[serde(rename = "comparedToMessage")]
    pub compared_to_message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApplicationTrendsRequest {
    #[serde(alias = "from")]
    pub from: Option<DateTime<Utc>>,
    #[serde(alias = "to")]
    pub to: Option<DateTime<Utc>>,
    pub statuses: Option<Vec<Status>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApplicationTrendsResponse {
    #[serde(alias = "bar_data")]
    pub bar_data: Vec<StatusCount>,

    #[serde(alias = "line_data")]
    pub line_data: Vec<DatesCount>,
}


#[derive(Serialize, Deserialize, ToSchema, FromRow)]
pub struct StatusCount {
    pub status: Status,
    pub count: i64,
}

#[derive(Serialize, Deserialize, ToSchema, FromRow)]
pub struct DatesCount {
    pub status: Status,
    pub date: DateTime<Utc>,
    pub count: i64,   
}


