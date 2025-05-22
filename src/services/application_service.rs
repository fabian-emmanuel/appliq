use crate::enums::application::Status;
use crate::errors::app_error::{extract_validation_errors, AppError};
use crate::models::application::{Application, ApplicationStatus};
use crate::payloads::application::{
    ApplicationFilter, ApplicationRequest, ApplicationStatusRequest, ApplicationStatusResponse,
    ApplicationsResponse,
};
use crate::payloads::pagination::PaginatedResponse;
use crate::repositories::application_repository::ApplicationRepository;
use std::sync::Arc;
use validator::Validate;

/// # Application Service
///
/// Provides business logic for managing job applications and their statuses.
/// It interacts with the `ApplicationRepository` to perform database operations.
pub struct ApplicationService {
    application_repo: Arc<ApplicationRepository>,
}

impl ApplicationService {
    /// Creates a new instance of `ApplicationService`.
    ///
    /// # Parameters
    /// - `application_repo`: An `Arc` wrapped `ApplicationRepository` for database interactions.
    ///
    /// # Returns
    /// An `Arc` wrapped `ApplicationService` instance.
    pub fn new(application_repo: Arc<ApplicationRepository>) -> Arc<Self> {
        Arc::new(Self { application_repo })
    }

    /// Creates a new job application.
    ///
    /// Validates the request, saves the application to the database, and then
    /// automatically adds an initial "Applied" status for the new application.
    ///
    /// # Parameters
    /// - `req`: The `ApplicationRequest` payload containing details of the application to create.
    /// - `user_id`: The ID of the user creating this application.
    ///
    /// # Returns
    /// - `Ok(ApplicationsResponse)`: Details of the created application, including its initial status.
    /// - `Err(AppError)`: An error if validation fails or a database operation fails.
    ///
    /// # Errors
    /// - `AppError::ValidationError` if the request payload is invalid.
    /// - `AppError::DatabaseError` if any database operation fails.
    pub async fn create_application(
        &self,
        req: ApplicationRequest,
        user_id: i64,
    ) -> Result<ApplicationsResponse, AppError> {
        // Validate the incoming request.
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        // Save the new application.
        let application = self
            .application_repo
            .save(Application::from_application_request(&req, user_id))
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create and save the default "Applied" status for the new application.
        let default_status = self
            .application_repo
            .save_application_status(ApplicationStatus::new(
                application.id,
                Status::Applied, // Default initial status
                None,            // No test type for initial status
                None,            // No interview type for initial status
                None,            // No specific notes for initial status
                user_id,
            ))
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Return the response containing the application and its initial status.
        Ok(ApplicationsResponse::from_application_and_status(
            &application,
            &vec![default_status],
        ))
    }

    /// Adds a new status to an existing job application.
    ///
    /// Checks if the application exists before adding the new status.
    ///
    /// # Parameters
    /// - `user_id`: The ID of the user adding the status (used as `created_by`).
    /// - `req`: The `ApplicationStatusRequest` payload containing details of the status to add.
    ///
    /// # Returns
    /// - `Ok(ApplicationStatusResponse)`: Details of the newly added status.
    /// - `Err(AppError)`: An error if the application is not found or a database operation fails.
    ///
    /// # Errors
    /// - `AppError::ResourceNotFound` if the specified application ID does not exist.
    /// - `AppError::DatabaseError` if any database operation fails.
    pub async fn add_application_status(
        &self,
        user_id: i64,
        req: ApplicationStatusRequest,
    ) -> Result<ApplicationStatusResponse, AppError> {
        // Verify that the application exists.
        match self
            .application_repo
            .exists_by_application_id(req.application_id)
            .await
        {
            Ok(false) => {
                return Err(AppError::ResourceNotFound(
                    "Application does not exist.".into(), // Corrected typo in message
                ));
            }
            Ok(true) => (), // Application exists, proceed.
            Err(e) => return Err(AppError::DatabaseError(e.to_string())),
        }

        // Save the new application status.
        self.application_repo
            .save_application_status(ApplicationStatus::from_application_status_request(
                &req, user_id,
            ))
            .await
            .map(|app_status| ApplicationStatusResponse::from_application_status(&app_status))
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Fetches job applications for a user with optional filters, providing a paginated response.
    ///
    /// Retrieves applications based on the provided filters and user ID. For each application,
    /// it also fetches its complete status history.
    ///
    /// # Parameters
    /// - `created_by`: The ID of the user whose applications are to be fetched.
    /// - `filter`: An `ApplicationFilter` struct containing filter criteria (search, status, date range, pagination).
    ///
    /// # Returns
    /// - `Ok(PaginatedResponse<ApplicationsResponse>)`: A paginated list of applications,
    ///   each including its full status history.
    /// - `Err(AppError)`: An error if any database operation fails.
    ///
    /// # Errors
    /// - `AppError::DatabaseError` if fetching applications or their statuses fails.
    pub async fn fetch_applications_for_user_with_filters(
        &self,
        created_by: i64,
        filter: ApplicationFilter,
    ) -> Result<PaginatedResponse<ApplicationsResponse>, AppError> {
        // Fetch the paginated list of base application data.
        let paginated_apps = self
            .application_repo
            .find_applications_by_user_with_filters(created_by, filter)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut application_responses: Vec<ApplicationsResponse> = Vec::new();

        // For each application, fetch its status history.
        for app in paginated_apps.items {
            let statuses = self
                .application_repo
                .find_all_statuses_by_application_id(app.id)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            // It's expected that an application always has at least one status (e.g., "Applied").
            // If not, it might indicate a data integrity issue or an edge case not handled during creation.
            // Here, we skip such applications from the response, but logging a warning might be beneficial.
            if statuses.is_empty() {
                // Consider logging a warning here for applications found without statuses.
                // e.g., warn!("Application with ID {} found without any status history.", app.id);
                continue; 
            }
            application_responses.push(ApplicationsResponse::from_application_and_status(
                &app, &statuses,
            ));
        }

        // Construct and return the paginated response.
        Ok(PaginatedResponse {
            items: application_responses,
            total_items: paginated_apps.total_items,
            page: paginated_apps.page,
            page_size: paginated_apps.page_size,
            total_pages: paginated_apps.total_pages,
        })
    }
}
