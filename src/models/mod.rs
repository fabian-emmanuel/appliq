//! # Data Models
//!
//! This module defines the core data structures that represent the application's
//! entities and their persistence in the database. These models are typically
//! used with `sqlx` for database interaction (deriving `FromRow`) and `serde`
//! for serialization/deserialization in API requests/responses.
//!
//! Many models also derive `utoipa::ToSchema` to be included in the OpenAPI
//! documentation.
//!
//! ## Submodules
//! - **`application`**: Contains `Application` and `ApplicationStatus` models,
//!   representing job applications and their status history.
//! - **`token`**: Defines the `Token` model, used for purposes like password resets.
//! - **`user`**: Defines the `User` model, representing application users.

pub mod user;
pub(crate) mod application;
pub(crate) mod token;