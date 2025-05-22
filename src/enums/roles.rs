use serde::{Deserialize, Serialize};
use sqlx::{Type};
use utoipa::ToSchema;

#[derive(Serialize, Type, Deserialize, Clone, ToSchema, Debug, PartialEq)]
#[sqlx(type_name = "VARCHAR")]
#[schema(description = "Defines the roles a user can have within the system.")]
pub(crate) enum Role {
    Admin,
    User
}