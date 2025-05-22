use crate::models::application::{Application, ApplicationStatus};
use crate::payloads::application::{
    ApplicationFilter, ApplicationStatusResponse, ApplicationsResponse,
};
use crate::payloads::pagination::{build_paginated_response, compute_pagination, count_with_filters, fetch_with_filters};
use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::collections::HashMap;
use std::sync::Arc;

/// # Application Repository
///
/// Handles database operations related to job applications and their statuses.
/// It provides methods for creating, querying, and managing application data
/// in the PostgreSQL database.
pub struct ApplicationRepository {
    /// Shared connection pool to the PostgreSQL database.
    pub pool: Arc<PgPool>,
}

impl ApplicationRepository {
    /// Creates a new instance of `ApplicationRepository`.
    ///
    /// # Parameters
    /// - `pool`: An `Arc<PgPool>` for database connectivity.
    ///
    /// # Returns
    /// An `Arc` wrapped `ApplicationRepository`.
    pub fn new(pool: Arc<PgPool>) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    /// Saves a new job application to the database.
    ///
    /// # Parameters
    /// - `application`: The `Application` model instance to save.
    ///
    /// # Returns
    /// - `Ok(Application)`: The saved application data, including its generated ID.
    /// - `Err(sqlx::Error)`: An error if the database insertion fails.
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

    /// Checks if an application exists in the database by its ID.
    ///
    /// # Parameters
    /// - `application_id`: The ID of the application to check.
    ///
    /// # Returns
    /// - `Ok(bool)`: `true` if an application with the given ID exists, `false` otherwise.
    /// - `Err(sqlx::Error)`: An error if the database query fails.
    pub async fn exists_by_application_id(&self, application_id: i64) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM applications WHERE id = $1 AND deleted = false)", // Added deleted = false
        )
        .bind(application_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(exists)
    }

    /// Saves a new status for a job application to the database.
    ///
    /// # Parameters
    /// - `application_status`: The `ApplicationStatus` model instance to save.
    ///
    /// # Returns
    /// - `Ok(ApplicationStatus)`: The saved application status data, including its generated ID.
    /// - `Err(sqlx::Error)`: An error if the database insertion fails.
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

    /// Fetches all status entries for a given application ID, ordered by creation date.
    ///
    /// # Parameters
    /// - `application_id`: The ID of the application for which to fetch statuses.
    ///
    /// # Returns
    /// - `Ok(Vec<ApplicationStatus>)`: A vector of `ApplicationStatus` instances.
    /// - `Err(sqlx::Error)`: An error if the database query fails.
    pub async fn find_all_statuses_by_application_id(
        &self,
        application_id: i64,
    ) -> Result<Vec<ApplicationStatus>, sqlx::Error> {
        sqlx::query_as::<_, ApplicationStatus>(
            r#"
            SELECT *
            FROM application_statuses
            WHERE application_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(application_id)
        .fetch_all(self.pool.as_ref())
        .await
    }
    
    /// Finds and paginates job applications for a specific user, applying various filters.
    ///
    /// This method constructs a dynamic SQL query based on the provided filters
    /// (search term, status, date range) and pagination parameters (page, size).
    /// It first counts the total matching records and then fetches the records for the current page.
    ///
    /// **Note:** The current implementation as read from `read_files` returns a `HashMap<String, Value>`.
    /// This was the state *before* a previous subtask that changed it to `PaginatedResponse<Application>`.
    /// This documentation reflects the provided file content. A separate task would be needed to align
    /// this method with the `PaginatedResponse<Application>` return type if that's the intended current state.
    ///
    /// # Parameters
    /// - `created_by`: The ID of the user whose applications are to be fetched.
    /// - `filter`: An `ApplicationFilter` struct containing filtering and pagination criteria.
    ///
    /// # Returns
    /// - `Ok(HashMap<String, Value>)`: A HashMap representing the paginated response,
    ///   containing keys like "data", "page", "total_items", "total_pages".
    /// - `Err(sqlx::Error)`: An error if any database query fails.
    pub async fn find_applications_by_user_with_filters(
        &self,
        created_by: i64,
        filter: ApplicationFilter,
    ) -> Result<HashMap<String, Value>, sqlx::Error> {
        // Count total matching applications based on filters.
        let total = count_with_filters(
            "SELECT COUNT(*) FROM applications", // Base query for counting.
            // Closure to apply filters to the count query.
            |b| self.apply_application_filters(b, filter.clone(), created_by.clone()),
            self.pool.as_ref(),
        )
        .await?;

        // Calculate pagination details (page, size, offset, total_pages).
        let (page, size, offset, total_pages) = compute_pagination(filter.page, filter.size, total);

        // Fetch the actual application data for the current page with filters.
        let applications: Vec<Application> = fetch_with_filters(
            "SELECT * FROM applications", // Base query for fetching applications.
            // Closure to apply filters to the fetch query.
            |b| self.apply_application_filters(b, filter.clone(), created_by), // filter is cloned for this call
            size,
            offset,
            self.pool.as_ref(),
        )
        .await?;

        // Fetch statuses for all retrieved applications in a single query for efficiency.
        let application_ids: Vec<i64> = applications.iter().map(|app| app.id).collect();
        let statuses_history: Vec<ApplicationStatus> = if !application_ids.is_empty() {
            sqlx::query_as::<_, ApplicationStatus>(
                r#"
                SELECT *
                FROM application_statuses
                WHERE application_id = ANY($1)
                ORDER BY application_id, created_at ASC
                "#,
            )
            .bind(&application_ids)
            .fetch_all(self.pool.as_ref())
            .await?
        } else {
            Vec::new() // No applications, so no statuses to fetch.
        };

        // Group statuses by application_id for easy lookup.
        let mut status_map: HashMap<i64, Vec<ApplicationStatusResponse>> = HashMap::new();
        for status_item in statuses_history {
            status_map
                .entry(status_item.application_id)
                .or_default()
                .push(ApplicationStatusResponse::from_application_status(&status_item));
        }

        // Combine application data with their respective status histories.
        let data: Vec<ApplicationsResponse> = applications
            .into_iter()
            .map(|app| {
                let current_status_history = status_map.get(&app.id).cloned().unwrap_or_default();
                let current_status_type = current_status_history.last().map(|s| s.status.clone())
                    .unwrap_or(crate::enums::application::Status::Applied); // Fallback if no status, though unlikely.

                ApplicationsResponse {
                    id: app.id,
                    company: app.company,
                    position: app.position,
                    website: app.website,
                    application_type: app.application_type,
                    created_at: app.created_at,
                    created_by: app.created_by,
                    status: current_status_type,
                    status_history: current_status_history,
                }
            })
            .collect();

        // Build and return the paginated response as a HashMap.
        Ok(build_paginated_response(data, page, total, total_pages, "applications"))
    }

    /// Applies filtering conditions to a SQL query for applications.
    ///
    /// This helper function is used by `find_applications_by_user_with_filters` to
    /// dynamically build the WHERE clause of the SQL query based on the provided filters.
    ///
    /// # Parameters
    /// - `builder`: A `QueryBuilder` instance to append conditions to.
    /// - `filter`: The `ApplicationFilter` containing filter criteria.
    /// - `created_by`: The ID of the user to filter applications for.
    ///
    /// # Returns
    /// The modified `QueryBuilder` with filter conditions applied.
    pub fn apply_application_filters<'a>(
        &self,
        mut builder: QueryBuilder<'a, Postgres>,
        filter: ApplicationFilter,
        created_by: i64,
    ) -> QueryBuilder<'a, Postgres> {
        // Always filter by the user who created the application and ensure not deleted.
        builder.push(" WHERE created_by = ").push_bind(created_by).push(" AND deleted = false");

        // Apply search filter for company, position, or website.
        if let Some(search) = filter.search {
            let pattern = format!("%{}%", search.trim()); // Trim search input
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
}
