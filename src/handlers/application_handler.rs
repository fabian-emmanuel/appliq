use crate::configs::routes::{ADD_APPLICATION, ADD_APPLICATION_STATUS, DELETE_APPLICATION, GET_APPLICATIONS_FOR_USER};
use crate::enums::application::Status;
use crate::errors::api_error::ApiError;
use crate::payloads::application::{
    ApplicationFilter, ApplicationRequest, ApplicationStatusRequest, ApplicationStatusResponse,
    ApplicationsResponse, UpdateApplicationRequest,
};
use crate::services::application_service::ApplicationService;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::Claims;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum_macros::debug_handler;
use http::StatusCode;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ApplicationHandler {
    pub application_service: Arc<ApplicationService>,
}

#[utoipa::path(post, path = ADD_APPLICATION, request_body = ApplicationRequest,
    responses(
        (status = 201, description = "Application successfully registered", body = ApiResponse<ApplicationsResponse>),
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
) -> Result<(StatusCode, Json<ApiResponse<ApplicationsResponse>>), (StatusCode, Json<ApiError>)> {
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
        (status = 200, description = "Status successfully added", body = ApiResponse<ApplicationStatusResponse>),
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
) -> Result<(StatusCode, Json<ApiResponse<ApplicationStatusResponse>>), (StatusCode, Json<ApiError>)>
{
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
        ("from" = Option<DateTime<Utc>>, Query, description = "Filter from this date (inclusive)"),
        ("to" = Option<DateTime<Utc>>, Query, description = "Filter to this date (inclusive)"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("size" = Option<i64>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Applications retrieved", body = HashMap<String, serde_json::Value>),
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
) -> Result<(StatusCode, Json<ApiResponse<HashMap<String, serde_json::Value>>>), (StatusCode, Json<ApiError>)>
{
    match handler
        .application_service
        .fetch_applications_for_user_with_filters(claims.subject, filter)
        .await
    {
        Ok(applications) => Ok((StatusCode::OK, Json(ApiResponse::new("Applications retrieved", applications)))),

        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(patch, path = "/api/v1/application/{id}", request_body = UpdateApplicationRequest, params(
        ("id" = String, Path, description = "Application ID to update")
    ),
    responses(
        (status = 200, description = "Application successfully updated", body = ApiResponse<ApplicationsResponse>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 404, description = "Application not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Application Handler",
    summary = "Update an application by ID")]
#[debug_handler]
pub async fn update_application(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims,
    Path(id): Path<i64>,
    Json(req): Json<UpdateApplicationRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ApplicationsResponse>>), (StatusCode, Json<ApiError>)> {
    match handler
        .application_service
        .update_application(claims.subject, id, req)
        .await
    {
        Ok(application_data) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new(
                "Application updated.",
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

#[utoipa::path(delete, path = DELETE_APPLICATION, params(
        ("id" = String, Path, description = "Application ID to delete")
    ),
    responses(
        (status = 200, description = "Application successfully deleted", body = ApiResponse<String>),
        (status = 404, description = "Application not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Application Handler",
    summary = "Delete an application by ID")]
#[debug_handler]
pub async fn delete_application(
    State(handler): State<Arc<ApplicationHandler>>,
    claims: Claims,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<ApiResponse<String>>), (StatusCode, Json<ApiError>)> {
    match handler
        .application_service
        .delete_application(claims.subject, id)
        .await
    {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new(
                "Application deleted successfully.",
                String::from(""),
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
