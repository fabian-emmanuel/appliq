use crate::models::application::{Application, ApplicationStatus};
use crate::payloads::application::{
    ApplicationFilter, ApplicationStatusResponse, ApplicationsResponse,
};
use crate::payloads::pagination::{
    PaginatedResponse, compute_pagination, count_with_filters, fetch_with_filters,
};
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ApplicationRepository {
    pub pool: Arc<PgPool>,
}

impl ApplicationRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn save(&self, application: Application) -> Result<Application, sqlx::Error> {
        sqlx::query_as::<_, Application>(
            r#"
                INSERT INTO applications (
                company, position, website, application_type,
                created_by, created_at, updated_at, deleted_at, deleted
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(&application.company)
        .bind(&application.position)
        .bind(&application.website)
        .bind(&application.application_type)
        .bind(&application.created_by)
        .bind(&application.created_at)
        .bind(&application.updated_at)
        .bind(&application.deleted_at)
        .bind(&application.deleted)
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn exists_by_application_id(&self, application_id: i64) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM applications WHERE id = $1)",
        )
        .bind(application_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(exists)
    }

    pub async fn save_application_status(
        &self,
        application_status: ApplicationStatus,
    ) -> Result<ApplicationStatus, sqlx::Error> {
        sqlx::query_as::<_, ApplicationStatus>(
            r#"
            INSERT INTO application_statuses(application_id, status_type, created_by, created_at, test_type, interview_type, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
            .bind(&application_status.application_id)
            .bind(&application_status.status_type)
            .bind(&application_status.created_by)
            .bind(&application_status.created_at)
            .bind(&application_status.test_type)
            .bind(&application_status.interview_type)
            .bind(&application_status.notes)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn find_applications_by_user_with_filters(
        &self,
        created_by: i64,
        filter: ApplicationFilter,
    ) -> Result<PaginatedResponse<ApplicationsResponse>, sqlx::Error> {
        let total = count_with_filters(
            "SELECT COUNT(*) FROM applications",
            |b| self.apply_application_filters(b, filter.clone(), created_by.clone()),
            self.pool.as_ref(),
        )
        .await?;

        let (page, size, offset, total_pages) = compute_pagination(filter.page, filter.size, total);

        let applications: Vec<Application> = fetch_with_filters(
            "SELECT * FROM applications",
            |b| self.apply_application_filters(b, filter, created_by),
            size,
            offset,
            self.pool.as_ref(),
        )
        .await?;

        // -------- FETCH STATUSES --------
        let application_ids: Vec<i64> = applications.iter().map(|app| app.id).collect();
        let statuses: Vec<ApplicationStatus> = sqlx::query_as::<_, ApplicationStatus>(
            r#"
        SELECT *
        FROM application_statuses
        WHERE application_id = ANY($1)
        ORDER BY created_at ASC
        "#,
        )
        .bind(&application_ids)
        .fetch_all(self.pool.as_ref())
        .await?;

        // -------- GROUP STATUSES --------
        let mut status_map: HashMap<i64, Vec<ApplicationStatusResponse>> = HashMap::new();
        for status in statuses {
            status_map
                .entry(status.application_id)
                .or_default()
                .push(ApplicationStatusResponse::from_application_status(&status));
        }

        // -------- COMBINE INTO ApplicationsResponse --------
        let data: Vec<ApplicationsResponse> = applications
            .into_iter()
            .map(|app| ApplicationsResponse {
                id: app.id,
                company: app.company,
                position: app.position,
                website: app.website,
                application_type: app.application_type,
                created_at: app.created_at,
                created_by: app.created_by,
                status: status_map
                    .get(&app.id)
                    .and_then(|statuses| statuses.last())
                    .map(|s| s.status.clone())
                    .unwrap(),
                status_history: status_map.remove(&app.id).unwrap_or_else(Vec::new),
            })
            .collect();
        let page_size = data.len() as i64;

        // -------- RETURN PAGINATED RESULT --------
        Ok(PaginatedResponse {
            data,
            total,
            total_pages,
            page,
            size,
            page_size,
        })
    }

    pub fn apply_application_filters<'a>(
        &self,
        mut builder: QueryBuilder<'a, Postgres>,
        filter: ApplicationFilter,
        created_by: i64,
    ) -> QueryBuilder<'a, Postgres> {
        builder.push(" WHERE created_by = ").push_bind(created_by);

        if let Some(search) = filter.search {
            let pattern = format!("%{}%", search);
            builder
                .push(" AND (company ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR position ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR website ILIKE ")
                .push_bind(pattern)
                .push(")");
        }

        if let Some(status) = filter.status {
            builder
                .push(" AND id IN (SELECT application_id FROM application_statuses WHERE status_type = ")
                .push_bind(status)
                .push(")");
        }

        if let Some(start) = filter.from {
            builder.push(" AND created_at >= ").push_bind(start);
        }

        if let Some(end) = filter.to {
            builder.push(" AND created_at <= ").push_bind(end);
        }

        builder
    }
}
