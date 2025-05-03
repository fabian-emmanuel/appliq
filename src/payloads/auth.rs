use serde::Deserialize;
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

