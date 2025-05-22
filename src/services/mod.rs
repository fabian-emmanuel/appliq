//! # Business Logic Services
//!
//! This module encapsulates the core business logic of the AppliQ application.
//! Each submodule is dedicated to a specific domain or set of related functionalities,
//! acting as an intermediary between the HTTP handlers and the data repositories.
//!
//! ## Submodules
//! - **`application_service`**: Manages job applications and their statuses, including
//!   creation, updates, and retrieval with filtering.
//! - **`auth_service`**: Handles user authentication, password management (reset, forgot),
//!   and JWT generation.
//! - **`email_service`**: Responsible for sending emails, such as password reset notifications.
//!   It configures SMTP transport and uses templates for email content.
//! - **`user_service`**: Manages user-related operations like registration and fetching
//!   user profile information.

pub(crate) mod user_service;
pub(crate) mod auth_service;
pub(crate) mod application_service;
pub(crate) mod email_service;