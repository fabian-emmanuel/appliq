use crate::models::user::User;
use sqlx::PgPool;
use std::sync::Arc;

/// # User Repository
///
/// Handles database operations related to `User` entities.
/// It provides methods for creating, querying, and updating user data
/// in the PostgreSQL database.
pub struct UserRepository {
    /// Shared connection pool to the PostgreSQL database.
    pub pool: Arc<PgPool>,
}

impl UserRepository {
    /// Creates a new instance of `UserRepository`.
    ///
    /// # Parameters
    /// - `pool`: An `Arc<PgPool>` for database connectivity.
    ///
    /// # Returns
    /// An `Arc` wrapped `UserRepository`.
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    /// Retrieves a user by their unique ID.
    ///
    /// # Parameters
    /// - `user_id`: The ID of the user to retrieve.
    ///
    /// # Returns
    /// - `Ok(User)`: The `User` data if found.
    /// - `Err(sqlx::Error)`: An error if the user is not found or the query fails.
    ///   Specifically, `sqlx::Error::RowNotFound` if no user with the ID exists.
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND deleted = false") // Ensure not soft-deleted
            .bind(user_id)
            .fetch_one(self.pool.as_ref())
            .await
    }

    /// Saves a new user to the database.
    ///
    /// # Parameters
    /// - `user`: The `User` model instance to save.
    ///
    /// # Returns
    /// - `Ok(User)`: The saved user data, including its generated ID and other defaults.
    /// - `Err(sqlx::Error)`: An error if the database insertion fails.
    pub async fn save(&self, user: User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                first_name, last_name, email, password, role, 
                created_at, updated_at, deleted_at, deleted, 
                is_verified, last_login_at, failed_login_attempts
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.role)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .bind(&user.deleted_at)
        .bind(&user.deleted)
        .bind(&user.is_verified)
        .bind(&user.last_login_at)
        .bind(&user.failed_login_attempts)
        .fetch_one(self.pool.as_ref())
        .await
    }

    /// Checks if a user exists with the given email address.
    ///
    /// # Parameters
    /// - `email`: The email address to check.
    ///
    /// # Returns
    /// - `Ok(bool)`: `true` if a user with the email exists and is not soft-deleted, `false` otherwise.
    /// - `Err(sqlx::Error)`: An error if the database query fails.
    pub async fn exists_by_email(&self, email: String) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND deleted = false)" // Ensure not soft-deleted
        )
        .bind(email)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(exists)
    }

    /// Retrieves a user by their email address.
    ///
    /// # Parameters
    /// - `email`: The email address of the user to retrieve.
    ///
    /// # Returns
    /// - `Ok(User)`: The `User` data if found and not soft-deleted.
    /// - `Err(sqlx::Error)`: An error if the user is not found or the query fails.
    ///   Specifically, `sqlx::Error::RowNotFound` if no user with the email exists.
    pub async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND deleted = false") // Ensure not soft-deleted
            .bind(email)
            .fetch_one(self.pool.as_ref())
            .await
    }

    /// Updates the password for a given user ID.
    ///
    /// Also updates the `updated_at` timestamp for the user.
    ///
    /// # Parameters
    /// - `user_id`: The ID of the user whose password is to be updated.
    /// - `password_hash`: The new, hashed password.
    ///
    /// # Returns
    /// - `Ok(())`: If the password was successfully updated.
    /// - `Err(sqlx::Error)`: An error if the database update fails.
    pub async fn update_password(&self, user_id: i64, password_hash: String) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET password = $1, updated_at = NOW() -- Use NOW() for PostgreSQL current timestamp
            WHERE id = $2 AND deleted = false -- Ensure not soft-deleted
            "#,
        )
        .bind(password_hash)
        .bind(user_id)
        .execute(&*self.pool) // Use &*self.pool for PgPool
        .await
        .map(|result| { // Check if any row was affected to confirm user existed and was updated.
            if result.rows_affected() == 0 {
                // This could indicate the user_id didn't exist or was soft-deleted.
                // Depending on desired behavior, you might return a custom error here.
                // For now, matching existing behavior of not specifically erroring on 0 rows affected.
            }
            ()
        })
    }
}
