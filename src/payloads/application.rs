use crate::enums::application::{ApplicationType, InterviewType, Status, TestType};
use crate::models::application::{Application, ApplicationStatus};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// # Application Filter Payload
///
/// Defines the query parameters used for filtering and paginating job applications.
/// This struct is typically deserialized from URL query parameters.
///
/// The `#[schema]` attributes provide OpenAPI documentation for query parameters.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
#[schema(description = "Filters for querying job applications.")]
pub struct ApplicationFilter {
    /// Optional search term to filter applications by company name or position title.
    /// The search is typically case-insensitive and matches partial strings.
    #[schema(description = "Search term to filter by company or position.", example = "Software")]
    pub search: Option<String>,
    #[schema(description = "Filter applications by current status.")]
    pub status: Option<Status>,
    #[schema(description = "Filter applications created from this date (UTC).", example = "2023-01-01T00:00:00Z")]
    pub from: Option<DateTime<Utc>>,
    #[schema(description = "Filter applications created up to this date (UTC).", example = "2023-12-31T23:59:59Z")]
    pub to: Option<DateTime<Utc>>,
    #[schema(description = "Page number for pagination.", example = 1)]
    pub page: Option<i64>,
    /// Optional page number for pagination. Defaults to the first page if not provided.
    #[schema(description = "Page number for pagination.", example = 1)]
    pub page: Option<i64>,
    /// Optional number of items per page for pagination. Defaults to a system-defined page size if not provided.
    #[schema(description = "Number of items per page for pagination.", example = 10)]
    pub size: Option<i64>,
}

/// # Application Request Payload
///
/// Represents the data required to create or update a job application.
/// This struct is used as the request body for creating new applications.
/// It includes validation rules for fields like `company` and `position`.
///
/// The `#[schema]` attributes provide OpenAPI documentation for the request body.
#[derive(Validate, Deserialize, ToSchema)]
#[schema(description = "Payload for creating or updating a job application.")]
pub struct ApplicationRequest {
    /// Name of the company for the job application. Must not be empty.
    #[schema(description = "Name of the company.", example = "Innovatech")]
    #[validate(length(min = 1, message = "Company name cannot be empty"))]
    pub company: String,

    /// Position or title applied for. Must not be empty.
    #[schema(description = "Position applied for.", example = "Senior Developer")]
    #[validate(length(min = 1, message = "Position cannot be empty"))]
    pub position: String,

    /// Optional URL of the job posting or the company's career page.
    #[schema(description = "URL of the job posting or company website.", example = "https://innovatech.com/careers")]
    pub website: Option<String>,

    /// Method or platform used to submit the application (e.g., Email, Website).
    /// Serialized as `applicationType` in JSON.
    #[serde(rename = "applicationType")]
    #[schema(description = "Method used to submit the application.")]
    pub application_type: Option<ApplicationType>,
}

/// # Applications Response Payload
///
/// Represents the detailed information of a single job application when returned by the API.
/// This includes the application's core details along with its current status and
/// a complete history of all status changes.
///
/// The `#[schema]` attributes provide OpenAPI documentation for the response body.
#[derive(Serialize, Deserialize, ToSchema)]
#[schema(description = "Detailed information about a job application, including its status history.")]
pub struct ApplicationsResponse {
    /// Unique identifier of the application.
    #[schema(description = "Unique identifier for the application.", example = 1)]
    pub id: i64,
    /// Name of the company.
    #[schema(description = "Company name.", example = "Innovatech")]
    pub company: String,
    #[schema(description = "Position applied for.", example = "Senior Developer")]
    pub position: String,
    #[schema(description = "URL of the job posting or company website.", example = "https://innovatech.com/careers")]
    pub website: Option<String>,
    #[serde(rename = "applicationType")]
    #[schema(description = "Method used to submit the application.")]
    pub application_type: Option<ApplicationType>,
    #[serde(rename = "createdAt")]
    #[schema(description = "Timestamp of when the application was created.")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "createdBy")]
    #[schema(description = "ID of the user who created the application.", example = 101)]
    pub created_by: i64,
    #[schema(description = "Current status of the application.")]
    pub status: Status,
    /// Current overall status of the application (e.g., Applied, Interview, Rejected).
    /// This is typically the most recent status from the `status_history`.
    #[schema(description = "Current status of the application.")]
    pub status: Status,
    /// A chronological list of all status changes this application has undergone.
    /// Each entry provides details about a specific status event.
    #[serde(rename = "statusHistory")]
    #[schema(description = "Chronological history of status changes for the application.")]
    pub status_history: Vec<ApplicationStatusResponse>,
}

impl ApplicationsResponse {
    /// Creates an `ApplicationsResponse` from an `Application` model and its associated statuses.
    ///
    /// This constructor is responsible for determining the current overall `status`
    /// (usually the latest one) and mapping the `ApplicationStatus` models to
    /// `ApplicationStatusResponse` DTOs for the history.
    ///
    /// # Parameters
    /// - `application`: A reference to the core `Application` model.
    /// - `statuses`: A vector of `ApplicationStatus` models associated with the application,
    ///   expected to be sorted chronologically (oldest to newest).
    ///
    /// # Returns
    /// An `ApplicationsResponse` instance.
    ///
    /// # Panics
    /// Panics if the `statuses` vector is empty, as an application is expected to have
    /// at least one status (e.g., "Applied").
    pub fn from_application_and_status(
        application: &Application,
        statuses: &Vec<ApplicationStatus>,
    ) -> Self {
        // The current status is the type of the last status in the chronological history.
        // Panics if statuses is empty, which implies a data integrity issue (application should always have at least one status).
        let current_status = statuses.last().expect("Application must have at least one status.").status_type.clone();

        let history = statuses
            .iter()
            .map(|status_model| ApplicationStatusResponse::from_application_status(status_model))
            .collect();

        Self {
            id: application.id.clone(),
            company: application.company.clone(),
            position: application.position.clone(),
            website: application.website.clone(),
            application_type: application.application_type.clone(),
            created_at: application.created_at.clone(),
            created_by: application.created_by.clone(),
            status: current_status,
            status_history: history,
        }
    }
}

/// # Application Status Response Payload
///
/// Represents a single status event within an application's history when returned by the API.
///
/// The `#[schema]` attributes provide OpenAPI documentation for this component.
#[derive(Serialize, Deserialize, ToSchema)]
#[schema(description = "Represents a single status event in an application's history.")]
pub struct ApplicationStatusResponse {
    /// Unique identifier for this specific status entry.
    #[schema(description = "Unique identifier for this status entry.", example = 5)]
    pub id: i64,
    /// ID of the parent application this status belongs to. Serialized as `applicationId`.
    #[serde(rename = "applicationId")]
    #[schema(description = "ID of the application this status belongs to.", example = 1)]
    pub application_id: i64,
    #[serde(rename = "createdBy")]
    #[schema(description = "ID of the user who recorded this status.", example = 101)]
    pub created_by: i64,
    #[serde(rename = "status")]
    #[schema(description = "The status of the application at this point in time.")]
    pub status: Status,
    #[serde(rename = "createdAt")]
    #[schema(description = "Timestamp of when this status was recorded.")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "testType")]
    #[schema(description = "Type of test associated with this status, if any.")]
    pub test_type: Option<TestType>,
    #[serde(rename = "interviewType")]
    #[schema(description = "Type of interview associated with this status, if any.")]
    pub interview_type: Option<InterviewType>,
    /// Optional type of test associated with this status (e.g., Technical, English).
    #[serde(rename = "testType")]
    #[schema(description = "Type of test associated with this status, if any.")]
    pub test_type: Option<TestType>,
    /// Optional type of interview associated with this status (e.g., HR, Technical).
    #[serde(rename = "interviewType")]
    #[schema(description = "Type of interview associated with this status, if any.")]
    pub interview_type: Option<InterviewType>,
    /// Optional notes providing more details about this status event.
    #[schema(description = "Additional notes for this status update.", example = "Passed initial HR screening.")]
    pub notes: Option<String>,
}

impl ApplicationStatusResponse {
    /// Creates an `ApplicationStatusResponse` from an `ApplicationStatus` model.
    ///
    /// This is a direct mapping from the database model to the response DTO.
    ///
    /// # Parameters
    /// - `application_status`: A reference to the `ApplicationStatus` model.
    ///
    /// # Returns
    /// An `ApplicationStatusResponse` instance.
    pub fn from_application_status(application_status: &ApplicationStatus) -> Self {
        Self {
            id: application_status.id.clone(),
            application_id: application_status.application_id.clone(),
            created_by: application_status.created_by.clone(),
            status: application_status.status_type.clone(),
            created_at: application_status.created_at.clone(),
            test_type: application_status.test_type.clone(),
            interview_type: application_status.interview_type.clone(),
            notes: application_status.notes.clone(),
        }
    }
}

/// # Application Status Request Payload
///
/// Represents the data required to add a new status update to an existing job application.
/// This struct is used as the request body for adding application statuses.
///
/// The `#[schema]` attributes provide OpenAPI documentation for the request body.
#[derive(Validate, Deserialize, ToSchema)]
#[schema(description = "Payload for adding a new status to a job application.")]
pub struct ApplicationStatusRequest {
    /// ID of the job application to which this status update pertains. Serialized as `applicationId`.
    #[serde(rename = "applicationId")]
    #[schema(description = "ID of the application to update.", example = 1)]
    pub application_id: i64,
    #[serde(rename = "status")]
    #[schema(description = "The new status to add to the application.")]
    pub status_type: Status,
    #[serde(rename = "testType")]
    #[schema(description = "Type of test, if this status relates to a test.")]
    pub test_type: Option<TestType>,
    #[serde(rename = "interviewType")]
    #[schema(description = "Type of interview, if this status relates to an interview.")]
    pub interview_type: Option<InterviewType>,
    /// The type of status being recorded (e.g., Applied, Test, Interview). Serialized as `status`.
    #[serde(rename = "status")]
    #[schema(description = "The new status to add to the application.")]
    pub status_type: Status,
    /// Optional type of test, relevant if `status_type` is `Test`. Serialized as `testType`.
    #[serde(rename = "testType")]
    #[schema(description = "Type of test, if this status relates to a test.")]
    pub test_type: Option<TestType>,
    /// Optional type of interview, relevant if `status_type` is `Interview`. Serialized as `interviewType`.
    #[serde(rename = "interviewType")]
    #[schema(description = "Type of interview, if this status relates to an interview.")]
    pub interview_type: Option<InterviewType>,
    /// Optional notes providing context for this status update.
    #[schema(description = "Notes accompanying this status update.", example = "Technical interview scheduled for next week.")]
    pub notes: Option<String>,
}
