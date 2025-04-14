use crate::errors::app_error::AppError;
use crate::utils::jwt::{validate_jwt, Claims};
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::{IntoResponse, Response},
};

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth_header) = parts.headers.get(AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    return match validate_jwt(token) {
                        Ok(claims) => Ok(claims),
                        Err(e) => Err(AppError::InvalidToken(e.to_string())),
                    };
                }
            }
        }
        Err(AppError::MissingToken(String::from("Authorization token is missing.")))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::MissingToken(msg) => (
                StatusCode::FORBIDDEN, format!("{msg}"),
            ),
            AppError::InvalidToken(msg) => (
                StatusCode::UNAUTHORIZED,
                format!("{msg}"),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", self),
            ),
        }
            .into_response()
    }
}
