use thiserror::Error;
use http::StatusCode;
use sqlx::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;
use crate::errors::api_error::ApiError;

#[derive(Error, Debug, ToSchema)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Resource already exists: {0}")]
    ResourceExists(String),

    #[error("Invalid request detected: {0}")]
    BadRequest(String),

    #[error("Resource could not be found: {0}")]
    ResourceNotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Authentication error: {0}")]
    MissingToken(String),

    #[error("Authentication error: {0}")]
    InvalidToken(String),
}

impl AppError {
    pub fn to_api_error(&self) -> ApiError {
        match self {
            AppError::DatabaseError(msg) => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("Database error: {}", msg),
            },
            AppError::ValidationError(msg) => ApiError {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Validation error: {}", msg),
            },
            AppError::AuthError(msg) => ApiError {
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                message: format!("Authentication error: {}", msg),
            },
            AppError::ResourceExists(msg) => ApiError {
                status_code: StatusCode::CONFLICT.as_u16(),
                message: format!("Resource already exists: {}", msg),
            },
            AppError::BadRequest(msg) => ApiError {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Invalid request: {}", msg),
            },
            AppError::ResourceNotFound(msg) => ApiError {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                message: format!("Resource not found: {}", msg),
            },
            AppError::InternalServerError(msg) => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("Internal server error: {}", msg),
            },
            AppError::MissingToken(msg) => ApiError {
                status_code: StatusCode::FORBIDDEN.as_u16(),
                message: format!("Missing Authorization Token: {}", msg),
            },
            AppError::InvalidToken(msg) => ApiError {
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                message: format!("Invalid Authorization Token: {}", msg),
            },
        }
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        AppError::DatabaseError(error.to_string())  // Convert the `sqlx::Error` into a string
    }
}


pub fn extract_validation_errors(errors: &ValidationErrors) -> String {
    errors
        .field_errors()
        .values()
        .flat_map(|errors| {
            errors.iter().map(|e| e.message.clone().unwrap_or_default())
        })
        .collect::<Vec<_>>()
        .join(", ")
}