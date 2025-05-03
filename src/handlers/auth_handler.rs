use crate::configs::routes::LOGIN;
use crate::errors::api_error::ApiError;
use crate::payloads::auth::LoginRequest;
use crate::services::auth_service::AuthService;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::Token;
use axum::Json;
use axum::extract::State;
use http::StatusCode;
use std::sync::Arc;
use tracing::error;

pub struct AuthHandler {
    pub auth_service: Arc<AuthService>,
}

#[utoipa::path(post, path = LOGIN, request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<Token>),
        (status = 400, description = "Invalid credentials", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Handler",
    summary = "Login user")]
pub async fn login(
    State(handler): State<Arc<AuthHandler>>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<ApiResponse<Token>>), (StatusCode, Json<ApiError>)> {
    match handler.auth_service.login(req).await {
        Ok(token) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("Login successful.", token)),
        )),

        Err(err) => {
            error!("Failed to Login user: {err}");
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            Err((status_code, Json(api_error)))
        }
    }
}
