//! # HTTP Request Handlers
//!
//! This module contains all the HTTP request handlers for the AppliQ application.
//! Handlers are responsible for:
//! - Parsing incoming requests (JSON bodies, query parameters, path parameters).
//! - Extracting necessary information, such as JWT claims for authenticated routes.
//! - Calling the appropriate service layer methods to perform business logic.
//! - Formatting responses (success or error) and sending them back to the client.
//!
//! Each submodule typically corresponds to a specific domain or resource:
//! - **`application_handler`**: Handles requests related to job applications (e.g., create, list, update status).
//! - **`auth_handler`**: Manages authentication requests (e.g., login, forgot password, reset password).
//! - **`user_handler`**: Deals with user-specific requests (e.g., user registration, fetching user data).
//!
//! Handlers make use of Axum extractors (e.g., `State`, `Json`, `Query`, `Claims`) and
//! `utoipa` macros for OpenAPI documentation generation.

pub(crate) mod user_handler;
pub(crate) mod auth_handler;
pub(crate) mod application_handler;
