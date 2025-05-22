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

/// # User Handler
///
/// This struct encapsulates the HTTP handler logic for user-related endpoints,
/// such as user registration and fetching user data. It holds a reference to the
/// `UserService` to delegate the core business logic for these operations.
pub struct UserHandler {
    /// Shared reference to the user service.
    pub user_service: Arc<UserService>,
}

/// Handles new user registration requests.
///
/// Receives a `UserRequest` in the request body, which contains the necessary
/// information for creating a new user (name, email, password, etc.).
/// This information is then passed to the `UserService` to perform the registration.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint.
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
    State(handler): State<Arc<UserHandler>>, // Access to the UserHandler state.
    Json(req): Json<UserRequest>,           // Parsed UserRequest from the JSON body.
) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiError>)> {
    // Delegate the registration logic to the user service.
    match handler.user_service.register_user(req).await {
        Ok(user) => {
            // On successful registration, return 201 Created with the new user's information.
            Ok((
                StatusCode::CREATED,
                Json(ApiResponse::new("User registered successfully", user)),
            ))
        }
        Err(err) => {
            // Log the specific error for internal review.
            error!("Failed to register user: {}", err);
            // Convert the service-level AppError into an API-level ApiError for the response.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}

/// Handles requests to fetch the authenticated user's profile information.
///
/// This endpoint is protected and requires JWT authentication. The user's ID is
/// extracted from the JWT claims (`Claims` extractor) and used to fetch the
/// user's data via the `UserService`.
///
/// The `utoipa::path` macro provides OpenAPI documentation for this endpoint.
#[utoipa::path(get, path = USER_DATA, responses(
        (status = 200, description = "User data retrieved successfully", body = ApiResponse<UserInfo>),
        (status = 401, description = "Unauthorized - invalid or expired token", body = ApiError),
        (status = 404, description = "User not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError),
    ),
    security(
        ("JWT" = [])
    ),
    tag = "User Handler",
    operation_id = "getUserData",
    summary = "Get authenticated user data",
    description = "Fetches the profile information of the currently authenticated user.")]
pub async fn get_user_data(
    State(handler): State<Arc<UserHandler>>, // Access to the UserHandler state.
    claims: Claims,                         // Extracted JWT claims for the authenticated user.
) -> Result<(StatusCode, Json<ApiResponse<UserInfo>>), (StatusCode, Json<ApiError>)> {
    // Delegate to the user service, using the user ID from the JWT claims.
    match handler.user_service.get_user_data(claims.subject).await {
        Ok(user) => {
            // On success, return 200 OK with the user's information.
            Ok((
                StatusCode::OK,
                Json(ApiResponse::new("User data retrieved", user)),
            ))
        }
        Err(err) => {
            // Log the specific error for internal review.
            error!("Failed to retrieve user data: {}", err);
            // Convert AppError to ApiError.
            let api_error = err.to_api_error();
            let status_code = StatusCode::from_u16(api_error.status_code)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            Err((status_code, Json(api_error)))
        }
    }
}
