use crate::models::application::{Application, ApplicationStatus};
use crate::payloads::application::{
    ApplicationFilter, ApplicationStatusResponse, ApplicationsResponse,
};
use crate::payloads::dashboard::{
    ApplicationTrendsRequest, ApplicationTrendsResponse, AverageResponseTime, DashboardCount,
    DatesCount, RecentActivitiesResponse, RecentActivity, StatusCount, SuccessRate,
};
use crate::payloads::pagination::{
    build_paginated_response, compute_pagination, count_with_filters, fetch_with_filters,
};
use bigdecimal::{BigDecimal, ToPrimitive};
use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use std::collections::HashMap;
use std::sync::Arc;
use chrono::Datelike;

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
        Ok(build_paginated_response(
            data,
            page,
            total,
            total_pages,
            "applications",
        ))
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
        let row = sqlx::query(
            r#"
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
        "#,
        )
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
            format!(
                "{:.2}%",
                (successful_count as f64 / total_count as f64) * 100.0
            )
        } else {
            "0.00%".to_string()
        };

        Ok(SuccessRate {
            percentage,
            message: "based on last 30 applications".to_string(),
        })
    }

    pub async fn get_chart_data(
        &self,
        user_id: i64,
        req: ApplicationTrendsRequest,
    ) -> Result<ApplicationTrendsResponse, sqlx::Error> {
        // 1. First query - ensure all status types are returned with count 0 if no applications
        let mut bar_query = QueryBuilder::new(
            r#"
        WITH all_statuses AS (
            SELECT unnest(ARRAY['Applied', 'Test', 'Interview', 'OfferAwarded', 'Rejected', 'Withdrawn']::VARCHAR[]) as status_type
        ),
        latest_statuses AS (
            SELECT DISTINCT ON (a.id)
                a.id as application_id,
                ast.status_type as status_type
            FROM applications a
            LEFT JOIN application_statuses ast ON a.id = ast.application_id
            WHERE a.created_by = $1 AND a.deleted = false
            ORDER BY a.id, ast.created_at DESC NULLS LAST
        )
        SELECT 
            all_statuses.status_type as status, 
            COALESCE(COUNT(latest_statuses.application_id), 0) as count
        FROM all_statuses
        LEFT JOIN latest_statuses ON all_statuses.status_type = latest_statuses.status_type::VARCHAR
        GROUP BY all_statuses.status_type
        ORDER BY all_statuses.status_type
        "#,
        );

        // 2. Second query - ensure all days in range and all statuses are returned
        let mut line_query = QueryBuilder::new(
            r#"
        WITH date_series AS (
            SELECT 
                generate_series(
                    date_trunc('day', COALESCE($2::timestamptz, date_trunc('month', CURRENT_DATE))),
                    date_trunc('day', COALESCE($3::timestamptz, CURRENT_DATE)),
                    interval '1 day'
                )::date as date
        ),
        all_statuses AS (
            SELECT unnest(ARRAY['Applied', 'Test', 'Interview', 'OfferAwarded', 'Rejected', 'Withdrawn']::VARCHAR[]) as status_type
        ),
        date_statuses AS (
            SELECT date_series.date, all_statuses.status_type
            FROM date_series
            CROSS JOIN all_statuses
        ),
        latest_statuses AS (
            SELECT DISTINCT ON (a.id)
                a.id as application_id,
                date_trunc('day', a.created_at) as created_date,
                ast.status_type as status_type
            FROM applications a
            LEFT JOIN application_statuses ast ON a.id = ast.application_id
            WHERE a.created_by = $1 AND a.deleted = false
            ORDER BY a.id, ast.created_at DESC NULLS LAST
        )
        SELECT 
            (date_statuses.date || ' 00:00:00')::TIMESTAMPTZ as date,
            date_statuses.status_type as status,
            COALESCE(COUNT(latest_statuses.application_id), 0) as count
        FROM date_statuses
        LEFT JOIN latest_statuses ON 
            date_statuses.date = latest_statuses.created_date AND
            date_statuses.status_type = latest_statuses.status_type::VARCHAR
        GROUP BY date_statuses.date, date_statuses.status_type
        ORDER BY date_statuses.date, date_statuses.status_type
        "#,
        );

        // Set default date range to current month if not provided
        let from = req.from.unwrap_or_else(|| {
            chrono::Local::now().with_day(1).unwrap().naive_local().and_utc()
        });
        let to = req.to.unwrap_or_else(|| chrono::Local::now().naive_local().and_utc());

        let bar_data: Vec<StatusCount> = bar_query
            .build_query_as()
            .bind(user_id)
            .fetch_all(self.pool.as_ref())
            .await?;

        let line_data: Vec<DatesCount> = line_query
            .build_query_as()
            .bind(user_id)
            .bind(from)
            .bind(to)
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(ApplicationTrendsResponse {
            bar_data,
            line_data,
        })
    }

    pub async fn compute_average_response_time(
        &self,
        user_id: i64,
    ) -> Result<AverageResponseTime, sqlx::Error> {
        let current_month_avg_days: Option<BigDecimal> = sqlx::query_scalar(
            r#"
            WITH applied_times AS (
                SELECT
                    application_id,
                    created_at AS applied_at
                FROM application_statuses
                WHERE status_type = 'Applied'
            ),
            response_times AS (
                SELECT
                    application_id,
                    MIN(created_at) AS responded_at
                FROM application_statuses
                WHERE status_type IN ('Test', 'Interview')
                GROUP BY application_id
            )
            SELECT
                FLOOR(AVG(EXTRACT(EPOCH FROM (rt.responded_at - at.applied_at)) / (60 * 60 * 24)))
            FROM applications a
            JOIN applied_times at ON a.id = at.application_id
            JOIN response_times rt ON a.id = rt.application_id
            WHERE a.created_by = $1
              AND a.deleted = FALSE
              AND rt.responded_at >= date_trunc('month', CURRENT_DATE)
              AND rt.responded_at < date_trunc('month', CURRENT_DATE) + interval '1 month'
              AND rt.responded_at > at.applied_at
            "#,
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        let previous_month_avg_days: Option<BigDecimal> = sqlx::query_scalar(
            r#"
            WITH applied_times AS (
                SELECT
                    application_id,
                    created_at AS applied_at
                FROM application_statuses
                WHERE status_type = 'Applied'
            ),
            response_times AS (
                SELECT
                    application_id,
                    MIN(created_at) AS responded_at
                FROM application_statuses
                WHERE status_type IN ('Test', 'Interview')
                GROUP BY application_id
            )
            SELECT
                FLOOR(AVG(EXTRACT(EPOCH FROM (rt.responded_at - at.applied_at)) / (60 * 60 * 24)))
            FROM applications a
            JOIN applied_times at ON a.id = at.application_id
            JOIN response_times rt ON a.id = rt.application_id
            WHERE a.created_by = $1
              AND a.deleted = FALSE
              AND rt.responded_at >= date_trunc('month', CURRENT_DATE - interval '1 month')
              AND rt.responded_at < date_trunc('month', CURRENT_DATE)
              AND rt.responded_at > at.applied_at
            "#,
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        let current_avg_i64 = current_month_avg_days.and_then(|bd| bd.to_i64());
        let previous_avg_i64 = previous_month_avg_days.and_then(|bd| bd.to_i64());

        let average = match current_avg_i64 {
            Some(days) => format!("{} days", days),
            None => "N/A".to_string(),
        };

        let (faster_message, compared_to_message) = match (current_avg_i64, previous_avg_i64) {
            (Some(current), Some(previous)) => {
                if current < previous {
                    (
                        format!("{} days faster", previous - current),
                        "Compared to last month".to_string(),
                    )
                } else if current > previous {
                    (
                        format!("{} days slower", current - previous),
                        "Compared to last month".to_string(),
                    )
                } else {
                    ("Same as last month".to_string(), "".to_string())
                }
            }
            _ => ("N/A".to_string(), "".to_string()),
        };

        Ok(AverageResponseTime {
            average,
            faster_message,
            compared_to_message,
        })
    }

    pub async fn get_recent_activities(
        &self,
        user_id: i64,
    ) -> Result<RecentActivitiesResponse, sqlx::Error> {
        let activities = sqlx::query_as::<_, RecentActivity>(
            r#"
        WITH recent_applications AS (
            SELECT
                a.id,
                a.company,
                a.position,
                (SELECT status_type FROM application_statuses WHERE application_id = a.id ORDER BY created_at ASC LIMIT 1) as "current_status",
                NULL as "previous_status",
                a.created_at as last_updated
            FROM applications a
            WHERE a.created_by = $1 AND a.deleted = FALSE
            ORDER BY a.created_at DESC
            LIMIT 6
        ),
        recent_status_updates AS (
            SELECT
                id,
                company,
                position,
                current_status,
                previous_status,
                last_updated
            FROM (
                SELECT
                    a.id,
                    a.company,
                    a.position,
                    ast.status_type as "current_status",
                    LAG(ast.status_type) OVER (PARTITION BY ast.application_id ORDER BY ast.created_at ASC) as "previous_status",
                    ast.created_at as last_updated
                FROM applications a
                JOIN application_statuses ast ON a.id = ast.application_id
                WHERE a.created_by = $1 AND a.deleted = FALSE
            ) AS subquery
            WHERE previous_status IS NOT NULL
            ORDER BY last_updated DESC
            LIMIT 6
        )
        SELECT * FROM recent_applications
        UNION ALL
        SELECT * FROM recent_status_updates
        ORDER BY last_updated DESC
        LIMIT 6
        "#,
        )
            .bind(user_id)
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(RecentActivitiesResponse { activities })
    }
}
