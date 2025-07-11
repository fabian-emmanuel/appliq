use crate::models::application::{Application, ApplicationStatus};
use crate::payloads::application::{
    ApplicationFilter, ApplicationStatusResponse, ApplicationsResponse,
};
use crate::payloads::pagination::{build_paginated_response, compute_pagination, count_with_filters, fetch_with_filters};
use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use std::collections::HashMap;
use std::sync::Arc;
use crate::payloads::dashboard::{ApplicationTrendsRequest, ApplicationTrendsResponse, DashboardCount, DatesCount, StatusCount, SuccessRate};

pub struct ApplicationRepository {
    pub pool: Arc<PgPool>,
}

impl ApplicationRepository {
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        Arc::new(Self { pool })
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
    ) -> Result<HashMap<String, Value>, sqlx::Error> {
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

        // -------- RETURN PAGINATED RESULT --------
        Ok(build_paginated_response(data, page, total, total_pages, "applications"))
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
                .push(" AND id IN (")
                .push("SELECT application_id FROM application_statuses AS s1 ")
                .push("WHERE status_type = ")
                .push_bind(status)
                .push(" AND created_at = (")
                .push("SELECT MAX(created_at) FROM application_statuses AS s2 ")
                .push("WHERE s2.application_id = s1.application_id")
                .push(")")
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

    pub async fn compute_stats(&self, created_by: i64) -> Result<DashboardCount, sqlx::Error> {
        let row = sqlx::query(r#"
            WITH latest_statuses AS (
                SELECT DISTINCT ON (a.id) 
                    a.id as application_id,
                    ast.status_type,
                    ast.test_type,
                    ast.interview_type
                FROM applications a
                LEFT JOIN application_statuses ast ON a.id = ast.application_id
                WHERE a.created_by = $1 AND a.deleted = false
                ORDER BY a.id, ast.created_at DESC NULLS LAST
            ),
            stats AS (
                SELECT 
                    COUNT(*) as total_applications,
                    COUNT(CASE WHEN status_type = 'Interview' THEN 1 END) as interviews,
                    COUNT(CASE WHEN status_type = 'Test' THEN 1 END) as tests,
                    COUNT(CASE WHEN status_type = 'OfferAwarded' THEN 1 END) as offers_awarded,
                    COUNT(CASE WHEN status_type = 'Withdrawn' THEN 1 END) as withdrawn,
                    COUNT(CASE WHEN status_type = 'Rejected' THEN 1 END) as rejected
                FROM latest_statuses
            )
            SELECT 
                total_applications,
                interviews,
                tests,
                offers_awarded,
                withdrawn,
                rejected
            FROM stats
        "#)
            .bind(created_by)
            .fetch_one(self.pool.as_ref())
            .await?;

        Ok(DashboardCount {
            total_applications: row.get("total_applications"),
            interviews: row.get("interviews"),
            tests: row.get("tests"),
            offers_awarded: row.get("offers_awarded"),
            withdrawn: row.get("withdrawn"),
            rejected: row.get("rejected"),
        })
    }

    pub async fn compute_success_rate(&self, created_by: i64) -> Result<SuccessRate, sqlx::Error> {
        let row = sqlx::query(r#"
            WITH latest_statuses AS (
                SELECT DISTINCT ON (a.id)
                    a.id as application_id,
                    ast.status_type
                FROM applications a
                LEFT JOIN application_statuses ast ON a.id = ast.application_id
                WHERE a.created_by = $1 AND a.deleted = false
                ORDER BY a.id, ast.created_at DESC NULLS LAST
            ),
            recent_applications AS (
                SELECT status_type
                FROM latest_statuses
                ORDER BY application_id DESC
                LIMIT 30
            ),
            successful_applications AS (
                SELECT COUNT(*) as count
                FROM recent_applications
                WHERE status_type = 'OfferAwarded' OR status_type = 'Interview' OR status_type = 'Test'
            ),
            total_applications AS (
                SELECT COUNT(*) as count
                FROM recent_applications
            )
            SELECT
                (SELECT count FROM successful_applications) as successful_count,
                (SELECT count FROM total_applications) as total_count
        "#)
            .bind(created_by)
            .fetch_one(self.pool.as_ref())
            .await?;

        let successful_count: i64 = row.get("successful_count");
        let total_count: i64 = row.get("total_count");

        let percentage = if total_count > 0 {
            format!("{:.2}%", (successful_count as f64 / total_count as f64) * 100.0)
        } else {
            "0.00%".to_string()
        };

        Ok(SuccessRate {
            percentage,
            message: "based on last 30 applications".to_string(),
        })
    }

    pub async fn get_chart_data(&self, user_id: i64, req: ApplicationTrendsRequest) -> Result<ApplicationTrendsResponse, sqlx::Error> {
        let mut bar_query = QueryBuilder::new(
            r#"
        WITH latest_statuses AS (
            SELECT DISTINCT ON (a.id)
                a.id as application_id,
                ast.status_type
            FROM applications a
            LEFT JOIN application_statuses ast ON a.id = ast.application_id
            WHERE a.created_by = $1 AND a.deleted = false
            ORDER BY a.id, ast.created_at DESC NULLS LAST
        )
        SELECT status_type as status, COUNT(*) as count
        FROM latest_statuses
        GROUP BY status_type
        "#
        );

        let mut line_query = QueryBuilder::new(
            r#"
        WITH latest_statuses AS (
            SELECT DISTINCT ON (a.id)
                a.id as application_id,
                a.created_at,
                ast.status_type
            FROM applications a
            LEFT JOIN application_statuses ast ON a.id = ast.application_id
            WHERE a.created_by = $1 AND a.deleted = false
            ORDER BY a.id, ast.created_at DESC NULLS LAST
        )
        SELECT 
            (DATE(created_at) || ' 00:00:00')::TIMESTAMPTZ as date, 
            status_type as status,
            COUNT(*) as count
        FROM latest_statuses
        WHERE 1=1
        "#
        );
        
        if let Some(from) = req.from {
            line_query.push(" AND created_at >= ").push_bind(from);
        }

        if let Some(to) = req.to {
            line_query.push(" AND created_at <= ").push_bind(to);
        }

        line_query.push(" GROUP BY (DATE(created_at) || ' 00:00:00')::TIMESTAMPTZ, status_type ORDER BY (DATE(created_at) || ' 00:00:00')::TIMESTAMPTZ, status_type");

        let bar_data: Vec<StatusCount> = bar_query
            .build_query_as()
            .bind(user_id)
            .fetch_all(self.pool.as_ref())
            .await?;

        let line_data: Vec<DatesCount> = line_query
            .build_query_as()
            .bind(user_id)
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(ApplicationTrendsResponse {
            bar_data,
            line_data,
        })
    }
}
