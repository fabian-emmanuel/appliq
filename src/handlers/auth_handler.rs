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

/// # Authentication Handler
///
/// This struct encapsulates the HTTP handler logic for authentication-related endpoints,
/// such as login, forgot password, and reset password. It holds a reference to the
/// `AuthService` to delegate the core business logic for these operations.
pub struct AuthHandler {
    /// Shared reference to the authentication service.
    pub auth_service: Arc<AuthService>,
}

/// Handles user login requests.
///
/// Takes a `LoginRequest` from the request body, passes it to the `AuthService`
/// for authentication, and returns a JWT token upon successful login.
/// If authentication fails, an appropriate error response is returned.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint.
#[utoipa::path(post, path = LOGIN, request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<JwtToken>),
        (status = 400, description = "Invalid credentials", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Handler",
    summary = "Login user",
    operation_id = "loginUser")]
pub async fn login(
    State(handler): State<Arc<AuthHandler>>, // Access to the AuthHandler state.
    Json(req): Json<LoginRequest>,          // Parsed LoginRequest from the JSON body.
) -> Result<(StatusCode, Json<ApiResponse<JwtToken>>), (StatusCode, Json<ApiError>)> {
    // Delegate the login logic to the authentication service.
    match handler.auth_service.login(req).await {
        Ok(token) => {
            // On successful login, return 200 OK with the JWT token.
            Ok((
                StatusCode::OK,
                Json(ApiResponse::new("Login successful.", token)),
            ))
        }
        Err(err) => {
            // Log the specific error for internal review.
            error!("Failed to Login user: {}", err);
            // Convert the service-level AppError into an API-level ApiError for the response.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

/// Handles requests to initiate the password reset process.
///
/// Receives a `ForgotPasswordRequest` (containing an email) in the request body.
/// It calls the `AuthService` to handle the logic of generating a reset token
/// and sending a password reset email.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint.
#[utoipa::path(post, path = FORGOT_PASSWORD, request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset instructions sent", body = ApiResponse<EmptyResponse>),
        (status = 400, description = "Invalid email format", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Handler",
    summary = "Request a password reset token",
    operation_id = "forgotPassword")]
#[debug_handler]
pub async fn forgot_password(
    State(handler): State<Arc<AuthHandler>>,
    Json(req): Json<ForgotPasswordRequest>, // Parsed ForgotPasswordRequest from JSON body.
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiError>)> {
    // Delegate to the authentication service.
    match handler.auth_service.forgot_password(req).await {
        Ok(_) => {
            // Return 200 OK with a generic message, regardless of whether the email exists,
            // to prevent email enumeration attacks.
            Ok((
                StatusCode::OK,
                Json(ApiResponse::new(
                    "If your email exists in our system, you will receive password reset instructions.",
                    (), // Empty response body.
                )),
            ))
        }
        Err(err) => {
            // Log the error internally.
            error!("Forgot password process failed: {}", err);
            // Convert AppError to ApiError.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

/// Handles requests to reset a user's password using a token.
///
/// Takes a `ResetPasswordRequest` (containing the new password, confirmation, and token)
/// from the request body. It calls the `AuthService` to validate the token,
/// update the password, and mark the token as used.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint.
#[utoipa::path(post, path = RESET_PASSWORD, request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password successfully reset", body = ApiResponse<EmptyResponse>),
        (status = 400, description = "Invalid request data or token", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "Auth Handler",
    summary = "Reset password using token",
    operation_id = "resetPassword")]
#[debug_handler]
pub async fn reset_password(
    State(handler): State<Arc<AuthHandler>>,
    Json(req): Json<ResetPasswordRequest>, // Parsed ResetPasswordRequest from JSON body.
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiError>)> {
    // Delegate to the authentication service.
    match handler.auth_service.reset_password(req).await {
        Ok(_) => {
            // Return 200 OK on successful password reset.
            Ok((
                StatusCode::OK,
                Json(ApiResponse::new("Password has been reset successfully.", ())), // Empty response body.
            ))
        }
        Err(err) => {
            // Log the error internally.
            error!("Reset password process failed: {}", err);
            // Convert AppError to ApiError.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}
