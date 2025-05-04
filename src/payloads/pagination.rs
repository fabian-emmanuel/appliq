use serde::Serialize;
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::collections::HashMap;

pub fn build_paginated_response<T: Serialize>(
    items: Vec<T>,
    page: i64,
    total: i64,
    total_pages: i64,
    key: &str
) -> HashMap<String, Value> {
    let size = items.len() as i64;

    // Create pagination map
    let pagination = serde_json::json!({
        "total": total,
        "size": size,
        "page": page,
        "totalPages": total_pages
    });

    // Create the final response map
    let mut data = HashMap::new();
    data.insert(key.to_string(), serde_json::to_value(items).unwrap_or_default());
    data.insert("pagination".to_string(), pagination);

    data
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

