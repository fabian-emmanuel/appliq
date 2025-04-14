use std::{net::SocketAddr, env};
use std::sync::Arc;
use tracing::{error, info};

mod configs;
mod models;
mod middlewares;
mod utils;
mod enums;
mod repositories;
mod services;
mod handlers;
mod errors;
mod payloads;

use crate::utils::custom_formatter::{init_tracing};

#[tokio::main]
async fn main() {
    init_tracing();
    info!("Starting server initialization...");

    let sqlx_pool = configs::database::establish_pool()
        .await
        .unwrap_or_else(|_| {
            std::process::exit(1);
        });

    sqlx::migrate!("db/migrations")
        .run(&sqlx_pool)
        .await
        .expect("Could Not Run Migrations");

    let host = env::var("SERVER_HOST").unwrap();
    let port = env::var("SERVER_PORT").unwrap();

    let app = configs::router::app_router(Arc::new(sqlx_pool));
    info!("Application router initialized.");

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address format");

    info!("Attempting to bind to address: {}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            listener
        }
        Err(e) => {
            error!("Failed to bind to {}: {}", addr, e);
            return;
        }
    };

    info!("Server is now running on port {}", port);
    if let Err(e) = axum::serve(listener, app).await {
        error!("Server encountered an error: {}", e);
    }
}

