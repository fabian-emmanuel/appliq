//! # API Route Constants
//!
//! This module defines constants for all API endpoint paths used in the application.
//! Using constants helps avoid typos and makes it easier to manage and update routes.
//! All routes are prefixed with `/api/v1`.

// --- Authentication Routes ---
/// Route for user login.
/// - Path: `/api/v1/login`
pub const LOGIN: &str = "/api/v1/login";

/// Route for initiating the password forgot/reset process.
/// - Path: `/api/v1/forgot-password`
pub const FORGOT_PASSWORD: &str = "/api/v1/forgot-password";

/// Route for resetting the password using a token.
/// - Path: `/api/v1/reset-password`
pub const RESET_PASSWORD: &str = "/api/v1/reset-password";

// --- User Routes ---
/// Route for fetching the authenticated user's data.
/// - Path: `/api/v1/user/me`
pub const USER_DATA: &str = "/api/v1/user/me";

/// Route for registering a new user.
/// - Path: `/api/v1/user/register`
pub const USER_REGISTER: &str = "/api/v1/user/register";

// --- Application Tracking Routes ---
/// Route for adding a new job application.
/// Also used for fetching all applications for the user (GET request to the same path).
/// - Path: `/api/v1/application`
pub const ADD_APPLICATION: &str = "/api/v1/application";

/// Route for fetching all job applications for the authenticated user.
/// This is the same path as `ADD_APPLICATION` but used with a GET request.
/// - Path: `/api/v1/application`
pub const GET_APPLICATIONS_FOR_USER: &str = "/api/v1/application";

/// Route for adding a status update to a specific job application.
/// - Path: `/api/v1/application/status`
pub const ADD_APPLICATION_STATUS: &str = "/api/v1/application/status";
