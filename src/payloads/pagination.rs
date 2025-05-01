use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Postgres, QueryBuilder};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,

    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    pub page: i64,
    pub size: i64,
    
    #[serde(rename = "pageSize")]
    pub page_size: i64,
}

pub fn compute_pagination(page: Option<i64>, size: Option<i64>, total: i64) -> (i64, i64, i64, i64) {
    let page = page.unwrap_or(1).max(1);
    let size = size.unwrap_or(20).max(1);
    let offset = (page - 1) * size;
    let total_pages = (total as f64 / size as f64).ceil() as i64;
    (page, size, offset, total_pages)
}


pub async fn count_with_filters(
    base_query: &str,
    apply_filters: impl FnOnce(QueryBuilder<'_, Postgres>) -> QueryBuilder<'_, Postgres>,
    pool: &PgPool,
) -> Result<i64, sqlx::Error> {
    let builder = QueryBuilder::new(base_query);
    let mut builder = apply_filters(builder);
    builder.build_query_scalar().fetch_one(pool).await
}


pub async fn fetch_with_filters<T>(
    base_query: &str,
    apply_filters: impl FnOnce(QueryBuilder<'_, Postgres>) -> QueryBuilder<'_, Postgres>,
    page_size: i64,
    offset: i64,
    pool: &PgPool,
) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Unpin + Send,
{
    let builder = QueryBuilder::new(base_query);
    let mut builder = apply_filters(builder);
    builder
        .push(" ORDER BY created_at DESC ")
        .push(" LIMIT ")
        .push_bind(page_size)
        .push(" OFFSET ")
        .push_bind(offset);

    builder.build_query_as::<T>().fetch_all(pool).await
}

