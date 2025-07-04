use crate::errors::app_error::AppError;
use crate::payloads::dashboard::{ApplicationTrendsRequest, ApplicationTrendsResponse, DashboardCount, SuccessRate};
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
    
    pub async fn compute_success_rate(&self, user_id: i64) -> Result<SuccessRate, AppError> {
        self
            .application_service
            .compute_success_rate(user_id)
            .await
            .map_err(AppError::from)
    }
    
    pub async fn get_chart_data(&self, user_id: i64, req: ApplicationTrendsRequest) -> Result<ApplicationTrendsResponse, AppError> {
        self.application_service
            .get_chart_data(user_id, req)
            .await
            .map_err(AppError::from)
    }
    
}