use crate::configs::routes::{USER_DATA, USER_REGISTER};
use crate::errors::api_error::ApiError;
use crate::payloads::user::{UserInfo, UserRequest};
use crate::services::user_service::UserService;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::Claims;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;
use tracing::error;

pub struct UserHandler {
    pub user_service: Arc<UserService>,
}

#[utoipa::path(post, path = USER_REGISTER, request_body = UserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponse<UserInfo>),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 409, description = "User already exists", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError),
    ),
    tag = "User Handler",
    operation_id = "registerUser",
    summary = "Register a new user",
    description = "Creates a new user account with the provided credentials and user information.")]
pub async fn register_user(
    State(handler): State<Arc<UserHandler>>,
    Json(req): Json<UserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiError>)> {
    match handler.user_service.register_user(req).await {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse::new("User registered successfully", user)),
        )),
        Err(err) => {
            error!("Failed to register user: {err}");
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            Err((status_code, Json(api_error)))
        }
    }
}

#[utoipa::path(get, path = USER_DATA, responses(
        (status = 200, description = "User data retrieved successfully", body = ApiResponse<UserInfo>),
        (status = 401, description = "Unauthorized - invalid or expired token", body = ApiError),
        (status = 404, description = "User not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User Handler",
    operation_id = "getUserData",
    summary = "Get authenticated user data",
    description = "Fetches the profile information of the currently authenticated user.")]
pub async fn get_user_data(
    State(handler): State<Arc<UserHandler>>,
    claims: Claims,
) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiError>)> {
    match handler.user_service.get_user_data(claims.subject).await {
        Ok(user) => Ok((
            StatusCode::OK,
            Json(ApiResponse::new("User data retrieved", user)),
        )),
        Err(err) => {
            error!("Failed to retrieve user data: {err}");
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            Err((status_code, Json(api_error)))
        }
    }
}
