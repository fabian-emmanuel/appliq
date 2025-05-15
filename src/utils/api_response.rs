use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(message: &str, data: T) -> Self {
        Self {
            message: String::from(message),
            data,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EmptyResponse {}

