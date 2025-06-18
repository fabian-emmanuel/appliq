use crate::models::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UserRepository {
    pub pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn save(&self, user: User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
        INSERT INTO users (first_name, last_name, email, password, role, created_at, updated_at, deleted_at, deleted, is_verified, last_login_at, failed_login_attempts, phone_number)
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
            .bind(&user.phone_number)
            .fetch_one(self.pool.as_ref())
            .await

    }


    pub async fn exists_by_email(&self, email: String) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
        )
            .bind(email)
            .fetch_one(self.pool.as_ref())
            .await?;

        Ok(exists)

    }


    pub async fn get_user_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn update_password(&self, user_id: i64, password_hash: String) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET password = $1, updated_at = NOW() AT TIME ZONE 'utc'
            WHERE id = $2
            "#,
        )
            .bind(password_hash)
            .bind(user_id)
            .execute(&*self.pool)
            .await
            .map(|_| ())
    }

}
