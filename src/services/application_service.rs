use crate::errors::app_error::{AppError, extract_validation_errors};
use crate::models::application::{
    Application, ApplicationData, ApplicationRequest, ApplicationStatus, ApplicationStatusData,
    ApplicationStatusRequest, ApplicationsResponse,
};
use crate::payloads::application::ApplicationFilter;
use crate::payloads::pagination::PaginatedResponse;
use crate::repositories::application_repository::ApplicationRepository;
use std::sync::Arc;
use validator::Validate;

pub struct ApplicationService {
    application_repo: Arc<ApplicationRepository>,
}

impl ApplicationService {
    pub fn new(application_repo: Arc<ApplicationRepository>) -> Self {
        Self { application_repo }
    }

    pub async fn create_application(
        &self,
        req: ApplicationRequest,
        user_id: i64,
    ) -> Result<ApplicationData, AppError> {
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        self.application_repo
            .save(Application::from_application_request(&req, user_id))
            .await
            .map(|application| ApplicationData::from_application(&application))
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn add_application_status(
        &self,
        user_id: i64,
        req: ApplicationStatusRequest,
    ) -> Result<ApplicationStatusData, AppError> {
        match self
            .application_repo
            .exists_by_application_id(req.application_id)
            .await
        {
            Ok(false) => {
                return Err(AppError::ResourceNotFound(
                    "Application does not exists.".into(),
                ));
            }
            Ok(true) => (),
            Err(e) => return Err(AppError::DatabaseError(e.to_string())),
        }

        self.application_repo
            .save_application_status(ApplicationStatus::from_application_status_request(
                &req, user_id,
            ))
            .await
            .map(|app_status| ApplicationStatusData::from_application_status(&app_status))
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn fetch_applications_for_user_with_filters(
        &self,
        created_by: i64,
        filter: ApplicationFilter,
    ) -> Result<PaginatedResponse<ApplicationsResponse>, AppError> {
        self.application_repo
            .find_applications_by_user_with_filters(created_by, filter)
            .await
            .map(|paginated_response| paginated_response)
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
