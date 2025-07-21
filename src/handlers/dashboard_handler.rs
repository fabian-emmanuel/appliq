use crate::configs::routes::{GET_AVERAGE_RESPONSE_TIME, GET_CHART_DATA, GET_DASHBOARD_STATS, GET_RECENT_ACTIVITIES, GET_SUCCESS_RATE};
use crate::errors::api_error::ApiError;
use crate::payloads::dashboard::{ApplicationTrendsRequest, ApplicationTrendsResponse, AverageResponseTime, DashboardCount, RecentActivitiesResponse, SuccessRate};
use crate::services::dashboard_service::DashboardService;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::Claims;
use axum::extract::{Query, State};
use axum::Json;
use axum_macros::debug_handler;
use http::StatusCode;
use std::sync::Arc;
use crate::enums::application::Status;

pub struct DashboardHandler {
    pub dashboard_service: Arc<DashboardService>,
}

#[utoipa::path(get, path = GET_DASHBOARD_STATS,
    responses(
        (status = 200, description = "Stats Retrieved.", body = ApiResponse<DashboardCount>),
        (status = 404, description = "User not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Dashboard Handler",
    summary = "Get dashboard stats")]
#[debug_handler]
pub async fn get_dashboard_stats(
    State(handler): State<Arc<DashboardHandler>>,
    claims: Claims,
) -> Result<(StatusCode, Json<ApiResponse<DashboardCount>>), (StatusCode, Json<ApiError>)> {
    match handler
        .dashboard_service
        .compute_dashboard_stats(claims.subject)
        .await
    {
        Ok(stats_data) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("Stats Retrieved.", stats_data)),
        )),
        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(get, path = GET_SUCCESS_RATE,
    responses(
        (status = 200, description = "Success Rate Retrieved.", body = ApiResponse<SuccessRate>),
        (status = 404, description = "User not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Dashboard Handler",
    summary = "Get success rate")]
#[debug_handler]
pub async fn get_success_rate(
    State(handler): State<Arc<DashboardHandler>>,
    claims: Claims,
) -> Result<(StatusCode, Json<ApiResponse<SuccessRate>>), (StatusCode, Json<ApiError>)> {
    match handler
        .dashboard_service
        .compute_success_rate(claims.subject)
        .await
    {
        Ok(success_rate) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("Success Rate Retrieved.", success_rate)),
        )),
        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(get, path = GET_CHART_DATA, params(
        ("statuses" = Option<Vec<Status>>, Query, description = "Filter by application statuses"),
        ("from" = Option<DateTime<Utc>>, Query, description = "Filter from this date (inclusive)"),
        ("to" = Option<DateTime<Utc>>, Query, description = "Filter to this date (inclusive)"),
    ),
    responses(
        (status = 200, description = "Retrieved.", body = ApiResponse<ApplicationTrendsResponse>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Dashboard Handler",
    summary = "Get chart data")]
#[debug_handler]
pub async fn get_chart_data(
    State(handler): State<Arc<DashboardHandler>>,
    claims: Claims,
    Query(req): Query<ApplicationTrendsRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ApplicationTrendsResponse>>), (StatusCode, Json<ApiError>)>
{
    match handler
        .dashboard_service
        .get_chart_data(claims.subject, req)
        .await
    {
        Ok(chart_data) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("Retrieved.", chart_data)),
        )),
        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(get, path = GET_AVERAGE_RESPONSE_TIME,
    responses(
        (status = 200, description = "Average response time retrieved.", body = ApiResponse<AverageResponseTime>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Dashboard Handler",
    summary = "Get average response time")]
#[debug_handler]
pub async fn get_average_response_time(
    State(handler): State<Arc<DashboardHandler>>,
    claims: Claims,
) -> Result<(StatusCode, Json<ApiResponse<AverageResponseTime>>), (StatusCode, Json<ApiError>)> {
    match handler
        .dashboard_service
        .compute_average_response_time(claims.subject)
        .await
    {
        Ok(average_response_time) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("Average response time retrieved.", average_response_time)),
        )),
        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(get, path = GET_RECENT_ACTIVITIES,
    responses(
        (status = 200, description = "Recent activities retrieved.", body = ApiResponse<RecentActivitiesResponse>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Dashboard Handler",
    summary = "Get recent activities")]
#[debug_handler]
pub async fn get_recent_activities(
    State(handler): State<Arc<DashboardHandler>>,
    claims: Claims,
) -> Result<(StatusCode, Json<ApiResponse<RecentActivitiesResponse>>), (StatusCode, Json<ApiError>)> {
    match handler
        .dashboard_service
        .get_recent_activities(claims.subject)
        .await
    {
        Ok(recent_activities) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("Recent activities retrieved.", recent_activities)),
        )),
        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}
