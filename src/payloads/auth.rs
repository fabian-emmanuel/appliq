use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct LoginRequest {

    #[validate(email(message = "Email must be valid"), length(min = 6))]
    pub(crate) email: String,

    pub(crate) password: String,

    #[serde(default, rename = "rememberMe")]
    pub(crate) remember_me: bool,

}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    #[serde(rename = "confirmPassword")]
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,

    pub token: String,
}
