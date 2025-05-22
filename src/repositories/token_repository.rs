use crate::models::token::Token;
use chrono::Local;
use sqlx::PgPool;
use std::sync::Arc;

/// # Token Repository
///
/// Manages database operations for `Token` entities, primarily used for
/// password resets and potentially other token-based verification processes.
pub struct TokenRepository {
    /// Shared connection pool to the PostgreSQL database.
    pool: Arc<PgPool>,
}

impl TokenRepository {
    /// Creates a new instance of `TokenRepository`.
    ///
    /// # Parameters
    /// - `pool`: An `Arc<PgPool>` for database connectivity.
    ///
    /// # Returns
    /// An `Arc` wrapped `TokenRepository`.
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    /// Saves a new token to the database.
    ///
    /// # Parameters
    /// - `token`: The `Token` model instance to save.
    ///
    /// # Returns
    /// - `Ok(Token)`: The saved token data, including its generated ID and other defaults.
    /// - `Err(sqlx::Error)`: An error if the database insertion fails.
    pub async fn save(&self, token: Token) -> Result<Token, sqlx::Error> {
        sqlx::query_as::<_, Token>(
            r#"
            INSERT INTO tokens (user_id, token, expires_at, created_at, used)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(&token.user_id)
        .bind(&token.token)
        .bind(&token.expires_at)
        .bind(&token.created_at)
        .bind(&token.used)
        .fetch_one(&*self.pool) // Use &*self.pool to get a reference to PgPool
        .await
    }

    /// Finds a token by its string value.
    ///
    /// This is typically used to retrieve a token during password reset verification.
    ///
    /// # Parameters
    /// - `token`: The string representation of the token to find.
    ///
    /// # Returns
    /// - `Ok(Some(Token))`: The found `Token` if it exists.
    /// - `Ok(None)`: If no token with the given string value is found.
    /// - `Err(sqlx::Error)`: An error if the database query fails.
    pub async fn find_by_token(&self, token: &str) -> Result<Option<Token>, sqlx::Error> {
        sqlx::query_as::<_, Token>(
            r#"
            SELECT id, user_id, token, expires_at, created_at, used
            FROM tokens
            WHERE token = $1
            "#,
        )
        .bind(token)
        .fetch_optional(&*self.pool) // Use &*self.pool
        .await
    }

    /// Marks a specific token as used.
    ///
    /// This is typically called after a token has been successfully used,
    /// for example, after a password reset.
    ///
    /// # Parameters
    /// - `token_id`: The ID of the token to mark as used.
    ///
    /// # Returns
    /// - `Ok(())`: If the token was successfully marked as used.
    /// - `Err(sqlx::Error)`: An error if the database update fails.
    pub async fn mark_as_used(&self, token_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE tokens
            SET used = true, updated_at = NOW() -- Also update updated_at timestamp
            WHERE id = $1
            "#,
        )
        .bind(token_id)
        .execute(&*self.pool) // Use &*self.pool
        .await
        .map(|_| ()) // Discard the Result<PgQueryResult, Error> and return Result<(), Error>
    }

    /// Invalidates all existing, non-expired, and unused tokens for a specific user.
    ///
    /// This is useful when a new password reset is requested, ensuring that
    /// only the latest token is valid. It marks them as `used = TRUE` and updates `updated_at`.
    ///
    /// # Parameters
    /// - `user_id`: The ID of the user whose tokens are to be invalidated.
    ///
    /// # Returns
    /// - `Ok(())`: If the operation was successful (even if no tokens were updated).
    /// - `Err(sqlx::Error)`: An error if the database update fails.
    pub async fn invalidate_existing_tokens_for_user(
        &self,
        user_id: i64,
    ) -> Result<(), sqlx::Error> {
        let now = Local::now(); // Current timestamp to compare against expires_at

        sqlx::query(
            r#"
            UPDATE tokens
            SET used = TRUE, updated_at = $1 -- Use provided 'now' for updated_at
            WHERE user_id = $2 AND used = FALSE AND expires_at > $1 -- Compare expires_at with 'now'
            "#,
        )
        .bind(now) // Bind 'now' for updated_at and for comparison with expires_at
        .bind(user_id)
        .execute(&*self.pool) // Use &*self.pool
        .await
        .map(|_| ())
    }
}
