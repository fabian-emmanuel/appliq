use serde::{Deserialize, Serialize};
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


#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ApplicationTrendsRequest {
    #[serde(alias = "startDate")]
    pub start_date: Option<String>,
    #[serde(alias = "endDate")]
    pub end_date: Option<String>,
    pub statuses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationTrendsResponse {
    #[serde(alias = "bar_data")]
    pub bar_data: Vec<StatusCount>,

    #[serde(alias = "line_data")]
    pub line_data: Vec<DatesCount>,
}


#[derive(Serialize, Deserialize)]
pub struct StatusCount {
    pub status: Status,
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DatesCount {
    pub date: String,
    pub count: i64,   
}


