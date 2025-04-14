use crate::enums::application::{ApplicationType, InterviewType, Status, TestType};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, PartialEq)]
pub struct Application {
    pub id: i64,
    pub company: String,
    pub position: String,
    pub website: Option<String>,
    pub application_type: ApplicationType,
    pub created_at: DateTime<Local>,
    pub created_by: i64,
    pub updated_at: DateTime<Local>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Local>>,
    #[serde(skip_serializing)]
    pub deleted: bool,
}


impl Application {
    pub fn new(
        company: String,
        position: String,
        website: Option<String>,
        application_type: ApplicationType,
        user_id: i64,
    ) -> Self {
        let now = Local::now();
        Self {
            id: 0,
            company,
            position,
            website,
            application_type,
            created_by: user_id,
            created_at: now,
            updated_at: now,
            deleted: false,
            deleted_at: None,
        }
    }

    pub fn from_application_request(request: &ApplicationRequest, user_id: i64) -> Self {
        Self::new(
            request.company.clone(),
            request.position.clone(),
            request.website.clone(),
            request.application_type.clone(),
            user_id,
        )
    }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ApplicationRequest {

    #[validate(length(min = 1, message = "Company name cannot be empty"))]
    pub company: String,

    #[validate(length(min = 1, message = "Position cannot be empty"))]
    pub position: String,

    pub website: Option<String>,

    pub application_type: ApplicationType,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, PartialEq, Decode)]
pub struct ApplicationStatus {
    pub id: i64,
    pub application_id: i64,
    pub status_type: Status,
    pub created_at: DateTime<Local>,
    pub created_by: i64,
    pub test_type: Option<TestType>,
    pub interview_type: Option<InterviewType>,
    pub notes: Option<String>,
}

impl ApplicationStatus {
    pub fn new(
        application_id: i64,
        status_type: Status,
        test_type: Option<TestType>,
        interview_type: Option<InterviewType>,
        notes: Option<String>,
        created_by: i64,
    ) -> Self {
        Self {
            id: 0,
            application_id,
            status_type,
            created_at: Local::now(),
            created_by,
            test_type,
            interview_type,
            notes,
        }
    }

    pub fn from_application_status_request(
        request: &ApplicationStatusRequest,
        user_id: i64,
    ) -> Self {
        Self::new(
            request.application_id.clone(),
            request.status_type.clone(),
            request.test_type.clone(),
            request.interview_type.clone(),
            request.notes.clone(),
            user_id
        )
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApplicationData {
    pub id: i64,
    pub company: String,
    pub position: String,
    pub website: Option<String>,
    pub application_type: ApplicationType,
    pub created_at: DateTime<Local>,
    pub created_by: i64,
}

impl ApplicationData {
    pub fn from_application(application: &Application) -> Self {
        Self {
            id: application.id.clone(),
            company: application.company.clone(),
            position: application.position.clone(),
            website: application.website.clone(),
            application_type: application.application_type.clone(),
            created_at: application.created_at.clone(),
            created_by: application.created_by.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApplicationsResponse {
    pub id: i64,
    pub company: String,
    pub position: String,
    pub website: Option<String>,
    pub application_type: ApplicationType,
    pub created_at: DateTime<Local>,
    pub created_by: i64,
    pub stages: Vec<ApplicationStatusData>,
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApplicationStatusData {
    pub id: i64,
    pub application_id: i64,
    pub created_by: i64,
    pub status_type: Status,
    pub created_at: DateTime<Local>,
    pub test_type: Option<TestType>,
    pub interview_type: Option<InterviewType>,
    pub notes: Option<String>,
}

impl ApplicationStatusData {
    pub fn from_application_status(application_status: &ApplicationStatus) -> Self {
        Self {
            id: application_status.id.clone(),
            application_id: application_status.application_id.clone(),
            created_by: application_status.created_by.clone(),
            status_type: application_status.status_type.clone(),
            created_at: application_status.created_at.clone(),
            test_type: application_status.test_type.clone(),
            interview_type: application_status.interview_type.clone(),
            notes: application_status.notes.clone(),
        }
    }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct ApplicationStatusRequest {
    pub application_id: i64,
    pub status_type: Status,
    pub test_type: Option<TestType>,
    pub interview_type: Option<InterviewType>,
    pub notes: Option<String>,
}




