use crate::enums::application::Status;
use crate::errors::app_error::{extract_validation_errors, AppError};
use crate::models::application::{Application, ApplicationStatus};
use crate::payloads::application::{
    ApplicationFilter, ApplicationRequest, ApplicationStatusRequest, ApplicationStatusResponse,
    ApplicationsResponse,
};
use crate::payloads::dashboard::{DashboardCount, SuccessRate};
use crate::repositories::application_repository::ApplicationRepository;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use validator::Validate;

pub struct ApplicationService {
    application_repo: Arc<ApplicationRepository>,
}

impl ApplicationService {
    pub fn new(application_repo: Arc<ApplicationRepository>) -> Arc<Self> {
        Arc::new(Self { application_repo })
    }

    pub async fn create_application(
        &self,
        req: ApplicationRequest,
        user_id: i64,
    ) -> Result<ApplicationsResponse, AppError> {
        req.validate()
            .map_err(|err| AppError::ValidationError(extract_validation_errors(&err)))?;

        let application = self
            .application_repo
            .save(Application::from_application_request(&req, user_id))
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let default_status = self
            .application_repo
            .save_application_status(ApplicationStatus::new(
                application.id,
                Status::Applied,
                None,
                None,
                None,
                user_id,
            ))
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(ApplicationsResponse::from_application_and_status(
            &application,
            &vec![default_status],
        ))
    }

    pub async fn add_application_status(
        &self,
        user_id: i64,
        req: ApplicationStatusRequest,
    ) -> Result<ApplicationStatusResponse, AppError> {
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
            .map(|app_status| ApplicationStatusResponse::from_application_status(&app_status))
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn fetch_applications_for_user_with_filters(
        &self,
        created_by: i64,
        filter: ApplicationFilter,
    ) -> Result<HashMap<String, Value>, AppError> {
        self.application_repo
            .find_applications_by_user_with_filters(created_by, filter)
            .await
            .map(|paginated_response| paginated_response)
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn compute_stats(&self, created_by: i64) -> Result<DashboardCount, AppError> {
        self.application_repo
            .compute_stats(created_by)
            .await
            .map(|stats| stats)
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn compute_success_rate(&self, created_by: i64) -> Result<SuccessRate, AppError> {
        self.application_repo
            .compute_success_rate(created_by)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
