//! # Data Repositories
//!
//! This module contains all data repository structures responsible for direct
//! interaction with the database. Repositories abstract the actual database queries
//! (SQL) and provide a clean API for services to access and manipulate data.
//!
//! Each submodule typically corresponds to a specific database table or a closely
//! related set of tables/entities:
//! - **`application_repository`**: Handles database operations for job applications
//!   and their statuses (e.g., creating applications, adding statuses, querying applications
//!   with filters).
//! - **`token_repository`**: Manages tokens stored in the database, primarily for
//!   password resets (e.g., saving tokens, finding tokens, marking them as used).
//! - **`user_repository`**: Deals with user data in the database (e.g., creating users,
//!   fetching users by ID or email, updating passwords).
//!
//! Repositories are designed to be used by the service layer and should not contain
//! business logic beyond what's necessary for data retrieval and persistence.

pub(crate) mod user_repository;
pub(crate) mod application_repository;
pub(crate) mod token_repository;