use crate::errors::app_error::AppError;
use crate::payloads::dashboard::DashboardCount;
use crate::services::application_service::ApplicationService;
use std::sync::Arc;

pub struct DashboardService {
    application_service: Arc<ApplicationService>
    
}

impl DashboardService {
    pub fn new(application_service: Arc<ApplicationService>) -> Arc<Self> {
        Arc::new(Self {application_service})
    }
    
    
    pub async fn compute_dashboard_stats(&self, user_id: i64) -> Result<DashboardCount, AppError> {
        self
            .application_service
            .compute_stats(user_id)
            .await
            .map(|stats | stats)
            .map_err(AppError::from)
    }
    
}