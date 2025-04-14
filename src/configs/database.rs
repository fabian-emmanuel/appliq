use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::{info, error};
use std::env;
use crate::errors::app_error::AppError;
use crate::errors::app_error::AppError::InternalServerError;

pub async fn establish_pool() -> Result<PgPool, AppError> {
    dotenvy::dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(err) => {
            error!("DATABASE_URL environment variable not set.");
            return Err(AppError::from(InternalServerError(err.to_string())));
        }
    };

    info!("Attempting to connect to the database...");

    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Successfully connected to the database.");
            Ok(pool)
        }
        Err(err) => {
            error!("Failed to connect to the database: {}", err);
            Err(AppError::from(err))
        }
    }
}


