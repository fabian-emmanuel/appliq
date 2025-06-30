use crate::errors::api_error::ApiError;
use crate::payloads::dashboard::{DashboardCount, SuccessRate};
use crate::services::dashboard_service::DashboardService;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::Claims;
use axum::extract::State;
use axum::Json;
use http::StatusCode;
use std::sync::Arc;
use axum_macros::debug_handler;
use crate::configs::routes::{GET_DASHBOARD_STATS, GET_SUCCESS_RATE};

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
pub async fn get_success_rate(State(handler): State<Arc<DashboardHandler>>,
                              claims: Claims
)-> Result<(StatusCode, Json<ApiResponse<SuccessRate>>), (StatusCode, Json<ApiError>)>{
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
