//! # Application Configuration Module
//!
//! This module centralizes all application-level configurations.
//! It includes submodules for:
//! - `api_doc`: OpenAPI (Swagger) documentation setup using `utoipa`.
//! - `database`: Database connection pooling and setup.
//! - `router`: Axum web router configuration, including route definitions and middleware.
//! - `routes`: Constants defining API endpoint paths.

pub(crate) mod database;
pub(crate) mod router;
mod api_doc;
pub(crate) mod routes;