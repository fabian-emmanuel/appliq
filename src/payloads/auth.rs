use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// # Login Request Payload
///
/// Represents the data required for a user to log in.
/// This struct is deserialized from the request body of the login endpoint.
/// It includes validation for the email format and length.
///
/// The `#[schema]` attributes provide OpenAPI documentation for the request body.
#[derive(Validate, Deserialize, ToSchema)]
#[schema(description = "Payload for user login.")]
pub struct LoginRequest {
    /// User's email address. Must be a valid email format and at least 6 characters long.
    #[schema(description = "User's email address.", example = "user@example.com")]
    #[validate(email(message = "Email must be valid"), length(min = 6))]
    pub(crate) email: String,

    /// User's raw password.
    #[schema(description = "User's password.", example = "password123")]
    pub(crate) password: String,

    /// Optional flag indicating whether the user wishes their session to be remembered
    /// for a longer period. Defaults to `false`.
    /// Serialized as `rememberMe` in JSON.
    #[serde(default, rename = "rememberMe")]
    #[schema(description = "Flag to remember the user session.", example = false)]
    pub(crate) remember_me: bool,
}

/// # Forgot Password Request Payload
///
/// Represents the data required to initiate a password reset process.
/// This struct is deserialized from the request body of the forgot password endpoint.
/// It includes validation for the email format.
///
/// The `#[schema]` attributes provide OpenAPI documentation for the request body.
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[schema(description = "Payload to request a password reset email.")]
pub struct ForgotPasswordRequest {
    /// Email address of the user requesting the password reset. Must be a valid email format.
    #[schema(description = "Email address of the user requesting password reset.", example = "user@example.com")]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// # Reset Password Request Payload
///
/// Represents the data required to reset a user's password using a token.
/// This struct is deserialized from the request body of the reset password endpoint.
/// It includes validation for password length and confirmation.
///
/// The `#[schema]` attributes provide OpenAPI documentation for the request body.
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[schema(description = "Payload to reset a user's password using a token.")]
pub struct ResetPasswordRequest {
    /// The new password for the user. Must be at least 8 characters long.
    #[schema(description = "New password for the user.", example = "newSecurePassword!")]
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    /// Confirmation of the new password. Must match the `password` field.
    /// Serialized as `confirmPassword` in JSON.
    #[serde(rename = "confirmPassword")]
    #[schema(description = "Confirmation of the new password.", example = "newSecurePassword!")]
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,

    /// The password reset token received by the user (typically via email).
    #[schema(description = "Password reset token received via email.", example = "reset_token_value")]
    pub token: String,
}
