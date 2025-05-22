use crate::configs::routes::{ADD_APPLICATION, ADD_APPLICATION_STATUS, GET_APPLICATIONS_FOR_USER};
use crate::enums::application::Status;
use crate::errors::api_error::ApiError;
use crate::enums::application::Status;
use crate::errors::api_error::ApiError;
use crate::payloads::application::{
    ApplicationFilter, ApplicationRequest, ApplicationStatusRequest, ApplicationStatusResponse,
    ApplicationsResponse,
};
use crate::payloads::pagination::PaginatedResponse;
use crate::services::application_service::ApplicationService;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::Claims;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use http::StatusCode;
use std::sync::Arc;

/// # Application Handler
///
/// This struct encapsulates the HTTP handler logic for job application-related endpoints.
/// It holds a reference to the `ApplicationService` to delegate business logic.
/// Each method corresponds to an API endpoint and handles request parsing,
/// calling the appropriate service method, and formatting the response.
pub struct ApplicationHandler {
    /// Shared reference to the application service for business logic.
    pub application_service: Arc<ApplicationService>,
}

/// Registers a new job application for the authenticated user.
///
/// This handler receives an `ApplicationRequest` in the request body.
/// It uses the `Claims` extractor to get the authenticated user's ID,
/// which is then passed to the `ApplicationService` to create the application.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint.
#[utoipa::path(post, path = ADD_APPLICATION, request_body = ApplicationRequest,
    responses(
        (status = 201, description = "Application successfully registered", body = ApiResponse<ApplicationsResponse>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("JWT" = [])
    ),
    tag = "Application Handler",
    summary = "Register a new job application",
    operation_id = "registerApplication")]
#[debug_handler]
pub async fn register_application(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims, // Extracted JWT claims for the authenticated user.
    Json(req): Json<ApplicationRequest>, // Parsed request body.
) -> Result<(StatusCode, Json<ApiResponse<ApplicationsResponse>>), (StatusCode, Json<ApiError>)> {
    // Delegate to the application service to create the application.
    // The user's ID from JWT claims is used as `created_by`.
    match handler
        .application_service
        .create_application(req, claims.subject)
        .await
    {
        Ok(application_data) => {
            // Successfully created, return 201 Created with application data.
            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::new(
                    "Application registered.",
                    application_data,
                )),
            ))
        }
        Err(err) => {
            // Convert the AppError from the service into an ApiError for the response.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

/// Adds a new status to an existing job application for the authenticated user.
///
/// This handler takes an `ApplicationStatusRequest` from the request body.
/// The `user_id` from the `Claims` is used to identify the user performing the action,
/// which is then passed to the `ApplicationService`.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint.
#[utoipa::path(post, path = ADD_APPLICATION_STATUS, request_body = ApplicationStatusRequest,
    responses(
        (status = 200, description = "Status successfully added", body = ApiResponse<ApplicationStatusResponse>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 404, description = "Application not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("JWT" = [])
    ),
    tag = "Application Handler",
    summary = "Add a new status to an application",
    operation_id = "addApplicationStatus")]
#[debug_handler]
pub async fn add_application_status(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims, // Extracted JWT claims.
    Json(req): Json<ApplicationStatusRequest>, // Parsed request body.
) -> Result<(StatusCode, Json<ApiResponse<ApplicationStatusResponse>>), (StatusCode, Json<ApiError>)>
{
    // Delegate to the application service to add the status.
    // The user's ID from JWT claims is used as `created_by` for the status.
    match handler
        .application_service
        .add_application_status(claims.subject, req)
        .await
    {
        Ok(application_status_data) => {
            // Successfully added, return 200 OK with new status data.
            Ok((
                StatusCode::OK,
                Json(ApiResponse::new(
                    "Application status added.",
                    application_status_data,
                )),
            ))
        }
        Err(err) => {
            // Convert AppError to ApiError for the response.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

/// Fetches job applications for the authenticated user, with optional filters and pagination.
///
/// This handler uses query parameters for filtering (`ApplicationFilter`) and pagination.
/// The `user_id` from `Claims` ensures that only applications belonging to the
/// authenticated user are retrieved.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint, including query parameters.
#[utoipa::path(get, path = GET_APPLICATIONS_FOR_USER, params(
        ("search" = Option<String>, Query, description = "Search by company or position"),
        ("status" = Option<Status>, Query, description = "Filter by application status"),
        ("from" = Option<DateTime<Utc>>, Query, description = "Filter from this date (inclusive)"),
        ("to" = Option<DateTime<Utc>>, Query, description = "Filter to this date (inclusive)"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("size" = Option<i64>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Applications retrieved successfully", body = ApiResponse<PaginatedResponse<ApplicationsResponse>>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("JWT" = [])
    ),
    tag = "Application Handler",
    summary = "Get user's applications with filters and pagination",
    operation_id = "fetchApplicationsForUserWithFilters")]
#[debug_handler]
pub async fn fetch_applications_for_user_with_filters(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims, // Extracted JWT claims.
    Query(filter): Query<ApplicationFilter>, // Parsed query parameters for filtering and pagination.
) -> Result<
    (StatusCode,Json<ApiResponse<PaginatedResponse<ApplicationsResponse>>>),
    (StatusCode, Json<ApiError>),
> {
    // Delegate to the application service to fetch applications.
    // The user's ID from JWT claims is used to scope the search.
    match handler
        .application_service
        .fetch_applications_for_user_with_filters(claims.subject, filter)
        .await
    {
        Ok(applications_response) => {
            // Successfully retrieved, return 200 OK with paginated application data.
            Ok((
                StatusCode::OK,
                Json(ApiResponse::new(
                    "Applications retrieved successfully.",
                    applications_response,
                )),
            ))
        }
        Err(err) => {
            // Convert AppError to ApiError for the response.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}
