use crate::configs::routes::{ADD_APPLICATION, ADD_APPLICATION_STATUS, GET_APPLICATIONS_FOR_USER};
use crate::enums::application::Status;
use crate::errors::api_error::ApiError;
use crate::models::application::{
    ApplicationData, ApplicationRequest, ApplicationStatusData, ApplicationStatusRequest,
    ApplicationsResponse,
};
use crate::payloads::application::ApplicationFilter;
use crate::payloads::pagination::PaginatedResponse;
use crate::services::application_service::ApplicationService;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::Claims;
use axum::Json;
use axum::extract::{Query, State};
use axum_macros::debug_handler;
use http::StatusCode;
use std::sync::Arc;

pub struct ApplicationHandler {
    pub application_service: Arc<ApplicationService>,
}

#[utoipa::path(post, path = ADD_APPLICATION, request_body = ApplicationRequest,
    responses(
        (status = 201, description = "Application successfully registered", body = ApiResponse<ApplicationData>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Application Handler",
    summary = "Register a new job application")]
#[debug_handler]
pub async fn register_application(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims,
    Json(req): Json<ApplicationRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ApplicationData>>), (StatusCode, Json<ApiError>)> {
    match handler
        .application_service
        .create_application(req, claims.subject)
        .await
    {
        Ok(application_data) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse::new(
                "Application registered.",
                application_data,
            )),
        )),

        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(post, path = ADD_APPLICATION_STATUS, request_body = ApplicationStatusRequest,
    responses(
        (status = 200, description = "Status successfully added", body = ApiResponse<ApplicationStatusData>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 404, description = "Application not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Application Handler",
    summary = "Add a new status to an application")]
#[debug_handler]
pub async fn add_application_status(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims,
    Json(req): Json<ApplicationStatusRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ApplicationStatusData>>), (StatusCode, Json<ApiError>)> {
    match handler
        .application_service
        .add_application_status(claims.subject, req)
        .await
    {
        Ok(application_status_data) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new(
                "Application status added.",
                application_status_data,
            )),
        )),

        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(get, path = GET_APPLICATIONS_FOR_USER, params(
        ("search" = Option<String>, Query, description = "Search by company or position"),
        ("status" = Option<Status>, Query, description = "Filter by application status"),
        ("start_date" = Option<DateTime<Utc>>, Query, description = "Filter from this date (inclusive)"),
        ("end_date" = Option<DateTime<Utc>>, Query, description = "Filter until this date (inclusive)"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("size" = Option<i64>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Applications retrieved", body = PaginatedResponse<ApplicationsResponse>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Application Handler",
    summary = "Get user's applications with filters and pagination")]
#[debug_handler]
pub async fn fetch_applications_for_user_with_filters(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims,
    Query(filter): Query<ApplicationFilter>,
) -> Result<(
    StatusCode,
    Json<PaginatedResponse<ApplicationsResponse>>,
    ),
    (StatusCode, Json<ApiError>),
> {
    match handler
        .application_service
        .fetch_applications_for_user_with_filters(claims.subject, filter)
        .await
    {
        Ok(applications) => Ok((
            StatusCode::OK,
            Json(applications),
        )),

        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}
