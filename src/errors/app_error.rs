use thiserror::Error;
use http::StatusCode;
use utoipa::ToSchema;
use validator::ValidationErrors;
use crate::errors::api_error::ApiError;
use tracing::error;

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

    #[error("Email error: {0}")]
    EmailError(String),

}

impl AppError {
    pub fn to_api_error(&self) -> ApiError {
        match self {
            AppError::DatabaseError(msg) => {
                error!("Database error: {:?}", msg);
                ApiError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "error returned from database".to_string(),
                }
            },
            AppError::ValidationError(msg) => ApiError {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: format!("{}", msg),
            },
            AppError::AuthError(msg) => ApiError {
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                message: format!("{}", msg),
            },
            AppError::ResourceExists(msg) => ApiError {
                status_code: StatusCode::CONFLICT.as_u16(),
                message: format!("{}", msg),
            },
            AppError::BadRequest(msg) => ApiError {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: format!("{}", msg),
            },
            AppError::ResourceNotFound(msg) => ApiError {
                status_code: StatusCode::NOT_FOUND.as_u16(),
                message: format!("{}", msg),
            },
            AppError::InternalServerError(msg) => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("{}", msg),
            },
            AppError::MissingToken(msg) => ApiError {
                status_code: StatusCode::FORBIDDEN.as_u16(),
                message: format!("{}", msg),
            },
            AppError::InvalidToken(msg) => ApiError {
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                message: format!("{}", msg),
            },
            AppError::EmailError(msg) => ApiError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("{}", msg),
            },
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        error!("Database error: {:?}", error);
        AppError::DatabaseError(error.to_string())
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