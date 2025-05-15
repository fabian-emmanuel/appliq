use crate::configs::routes::{LOGIN, FORGOT_PASSWORD, RESET_PASSWORD};
use crate::errors::api_error::ApiError;
use crate::payloads::auth::{LoginRequest, ForgotPasswordRequest, ResetPasswordRequest};
use crate::services::auth_service::AuthService;
use crate::utils::api_response::{ApiResponse, EmptyResponse};
use crate::utils::jwt::JwtToken;
use axum::Json;
use axum::extract::State;
use http::StatusCode;
use std::sync::Arc;
use axum_macros::debug_handler;
use tracing::error;

pub struct AuthHandler {
    pub auth_service: Arc<AuthService>,
}

#[utoipa::path(post, path = LOGIN, request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<JwtToken>),
        (status = 400, description = "Invalid credentials", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Handler",
    summary = "Login user")]
pub async fn login(
    State(handler): State<Arc<AuthHandler>>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<ApiResponse<JwtToken>>), (StatusCode, Json<ApiError>)> {
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

#[utoipa::path(post, path = FORGOT_PASSWORD, request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset instructions sent", body = ApiResponse<EmptyResponse>),
        (status = 400, description = "Invalid email format", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Handler",
    summary = "Request a password reset token")]
#[debug_handler]
pub async fn forgot_password(
    State(handler): State<Arc<AuthHandler>>,
    Json(req): Json<ForgotPasswordRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiError>)> {
    match handler.auth_service.forgot_password(req).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new(
                "If your email exists in our system, you will receive password reset instructions.",
                (),
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

#[utoipa::path(post, path = RESET_PASSWORD, request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password successfully reset", body = ApiResponse<EmptyResponse>),
        (status = 400, description = "Invalid request data or token", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Handler",
    summary = "Reset password using token")]
#[debug_handler]
pub async fn reset_password(
    State(handler): State<Arc<AuthHandler>>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiError>)> {
    match handler.auth_service.reset_password(req).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("Password has been reset successfully.", ())),
        )),
        Err(err) => {
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}
