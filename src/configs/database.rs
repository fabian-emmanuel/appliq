use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::{info, error};
use std::env;
use crate::errors::app_error::AppError;

/// Establishes a connection pool to the PostgreSQL database.
///
/// This function reads the `DATABASE_URL` environment variable, which should specify
/// the connection string for the PostgreSQL database. It then attempts to create a
/// connection pool with a maximum of 5 connections.
///
/// It ensures that `dotenvy::dotenv().ok()` is called to load environment variables
/// from a `.env` file if present.
///
/// # Returns
/// - `Ok(PgPool)`: A `PgPool` instance representing the database connection pool if successful.
/// - `Err(AppError)`: An `AppError` if:
///   - The `DATABASE_URL` environment variable is not set (returns `AppError::InternalServerError`).
///   - Connecting to the database fails (returns `AppError` converted from `sqlx::Error`).
///
/// # Errors
/// - Returns `AppError::InternalServerError` if the `DATABASE_URL` is not set.
/// - Returns an `AppError` wrapping an `sqlx::Error` if the connection attempt fails.
pub async fn establish_pool() -> Result<PgPool, AppError> {
    // Load environment variables from .env file, if present.
    dotenvy::dotenv().ok();

    // Retrieve the database URL from environment variables.
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Ok(url) => url,
        Err(e) => {
            error!("DATABASE_URL environment variable not set: {}", e);
            return Err(AppError::InternalServerError(
                "DATABASE_URL must be set".to_string(),
            ));
        }
    };

    info!("Attempting to connect to the database at the provided URL...");

    // Create a new PostgreSQL connection pool.
    match PgPoolOptions::new()
        .max_connections(5) // Configure the maximum number of connections in the pool.
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Successfully connected to the database and established connection pool.");
            Ok(pool)
        }
        Err(sqlx_err) => {
            error!("Failed to connect to the database: {}", sqlx_err);
            // Convert sqlx::Error to AppError.
            Err(AppError::DatabaseError(sqlx_err.to_string()))
        }
    }
}


