//! # AppliQ API Server
//!
//! This is the main entry point for the AppliQ application server.
//! It initializes the application, sets up database connections,
//! runs migrations, configures the router, and starts the HTTP server.

use std::{net::SocketAddr, sync::Arc};
use tracing::{error, info};

// Application modules
mod configs;
mod enums;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod payloads;
mod repositories;
mod services;
mod utils;

use crate::utils::custom_formatter::init_tracing;

/// The main asynchronous function that sets up and runs the AppliQ server.
///
/// This function performs the following steps:
/// 1. Initializes tracing for logging.
/// 2. Establishes a connection pool to the PostgreSQL database. Exits if connection fails.
/// 3. Runs database migrations. Panics if migrations fail.
/// 4. Initializes the application router with shared state (database pool).
/// 5. Parses the server port from the `PORT` environment variable (defaults to 3000).
/// 6. Binds a TCP listener to the specified address and port.
/// 7. Starts serving incoming HTTP requests using the configured Axum application.
///
/// # Panics
/// - Panics if database migrations cannot be run.
/// - Panics if the `PORT` environment variable cannot be parsed into a `u16`.
///
/// # Errors
/// - Logs an error and returns if the server fails to bind to the specified address.
/// - Logs an error if the server encounters an operational error after starting.
#[tokio::main]
async fn main() {
    // Initialize tracing for structured logging.
    init_tracing();
    info!("Starting server initialization...");

    // Establish a connection pool to the database.
    // Exits the process if the database connection cannot be established.
    let sqlx_pool = configs::database::establish_pool()
        .await
        .unwrap_or_else(|db_err| {
            error!("Failed to connect to the database: {}", db_err);
            std::process::exit(1);
        });
    info!("Database connection pool established.");

    // Run database migrations.
    // Panics if migrations fail, as the application cannot run without a correct schema.
    sqlx::migrate!("db/migrations")
        .run(&sqlx_pool)
        .await
        .expect("Could not run database migrations. Ensure the database is accessible and migrations are correct.");
    info!("Database migrations completed successfully.");

    // Initialize the application router, passing the database pool as shared state.
    let app = configs::router::app_router(Arc::new(sqlx_pool));
    info!("Application router initialized.");

    // Determine the server port from the PORT environment variable, defaulting to 3000.
    // Panics if the port string cannot be parsed.
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse PORT environment variable. Please provide a valid u16 port number.");
    
    // Create the server address (listen on all interfaces).
    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    info!("Attempting to bind to address: {}", addr);

    // Bind the TCP listener to the address.
    // Logs an error and exits if binding fails.
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Successfully bound to address: {}", addr);
            listener
        }
        Err(e) => {
            error!("Failed to bind to {}: {}. Ensure the port is not in use and permissions are correct.", addr, e);
            return; // Exit main if binding fails.
        }
    };

    info!("Server is now running on {}", addr);
    // Start serving requests. Logs an error if the server stops unexpectedly.
    if let Err(e) = axum::serve(listener, app).await {
        error!("Server encountered an error: {}", e);
    }
}

