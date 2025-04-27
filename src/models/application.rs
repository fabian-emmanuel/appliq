use crate::enums::application::{ApplicationType, InterviewType, Status, TestType};
use crate::payloads::application::{ApplicationRequest, ApplicationStatusRequest};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, PartialEq)]
pub struct Application {
    pub id: i64,
    pub company: String,
    pub position: String,
    pub website: Option<String>,
    pub application_type: Option<ApplicationType>,
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
        application_type: Option<ApplicationType>,
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
            user_id,
        )
    }
}
