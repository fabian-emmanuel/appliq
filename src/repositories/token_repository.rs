use crate::models::token::Token;
use chrono::Local;
use sqlx::PgPool;
use std::sync::Arc;

pub struct TokenRepository {
    pool: Arc<PgPool>,
}

impl TokenRepository {
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        Arc::new(Self { pool })
    }

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
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn find_by_token(&self, token: &str) -> Result<Option<Token>, sqlx::Error> {
        sqlx::query_as::<_, Token>(
            r#"
        SELECT id, user_id, token, expires_at, created_at, used
        FROM tokens
        WHERE token = $1
        "#,
        )
        .bind(token)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn mark_as_used(&self, token_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE tokens
        SET used = true
        WHERE id = $1
        "#,
        )
        .bind(token_id)
        .execute(&*self.pool)
        .await
        .map(|_| ())
    }

    pub async fn invalidate_existing_tokens_for_user(
        &self,
        user_id: i64,
    ) -> Result<(), sqlx::Error> {
        let now = Local::now();

        sqlx::query(
            r#"
            UPDATE tokens
            SET used = TRUE, updated_at = $1
            WHERE user_id = $2 AND used = FALSE AND expires_at > $1
            "#,
        )
        .bind(now)
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .map(|_| ())
    }
}
