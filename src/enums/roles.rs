use serde::{Deserialize, Serialize};
use sqlx::{Type};
use utoipa::ToSchema;

#[derive(Serialize, Type, Deserialize, Clone, ToSchema, Debug, PartialEq)]
#[sqlx(type_name = "VARCHAR")]
pub(crate) enum Role {
    Admin,
    User
}