use crate::configs::api_doc::ApiDoc;
use crate::configs::routes::{ADD_APPLICATION, ADD_APPLICATION_STATUS, DELETE_APPLICATION, FORGOT_PASSWORD, GET_APPLICATIONS_FOR_USER, GET_CHART_DATA, GET_DASHBOARD_STATS, GET_SUCCESS_RATE, LOGIN, LOGOUT, RESET_PASSWORD, USER_DATA, USER_REGISTER, GET_AVERAGE_RESPONSE_TIME, GET_RECENT_ACTIVITIES};
use crate::handlers::application_handler::{add_application_status, delete_application, fetch_applications_for_user_with_filters, register_application, ApplicationHandler};
use crate::handlers::auth_handler::{forgot_password, login, logout, reset_password, AuthHandler};
use crate::handlers::user_handler::{get_user_data, register_user, UserHandler};
use crate::repositories::application_repository::ApplicationRepository;
use crate::repositories::user_repository::UserRepository;
use crate::services::application_service::ApplicationService;
use crate::services::auth_service::AuthService;
use crate::services::user_service::UserService;
use axum::routing::{delete, get, post};
use axum::Router;
use dotenvy::var;
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::handlers::dashboard_handler::{get_average_response_time, get_chart_data, get_dashboard_stats, get_recent_activities, get_success_rate, DashboardHandler};
use crate::repositories::token_repository::TokenRepository;
use crate::services::dashboard_service::DashboardService;
use crate::services::email_service::EmailService;

pub fn app_router(db_pool: Arc<PgPool>) -> Router {
    
    let frontend_urls = var("FRONTEND_URLS").expect("FRONTEND_URLS must be set");
    
    let origins: Vec<_> = frontend_urls
        .split(',')
        .map(|url| url.parse().unwrap())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let user_repo = UserRepository::new(db_pool.clone());
    let token_repo = TokenRepository::new(db_pool.clone());
    let email_service = EmailService::new();
    
    let user_service = UserService::new(user_repo.clone());
    let user_handler = Arc::new(UserHandler {
        user_service: user_service.clone(),
    });
    let user_handler_router = Router::new()
        .route(USER_REGISTER, post(register_user))
        .route(USER_DATA, get(get_user_data))
        .with_state(user_handler);

    let auth_service = AuthService::new(user_repo.clone(), token_repo.clone(), email_service.clone());
    let auth_handler = Arc::new(AuthHandler { auth_service });
    let auth_handler_router = Router::new()
        .route(LOGIN, post(login))
        .route(FORGOT_PASSWORD, post(forgot_password))
        .route(RESET_PASSWORD, post(reset_password))
        .route(LOGOUT, post(logout))
    .with_state(auth_handler);

    let swagger_router = Router::new()
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()));


    let application_repo = ApplicationRepository::new(db_pool.clone());
    let application_service = ApplicationService::new(application_repo);
    let application_handler = Arc::new(ApplicationHandler {application_service: application_service.clone()});
    let application_handler_router = Router::new()
        .route(ADD_APPLICATION, post(register_application))
        .route(ADD_APPLICATION_STATUS, post(add_application_status))
        .route(GET_APPLICATIONS_FOR_USER, get(fetch_applications_for_user_with_filters))
        .route(DELETE_APPLICATION, delete(delete_application))
        .with_state(application_handler);
    
    let dashboard_service = DashboardService::new(application_service);
    let dashboard_handler = Arc::new(DashboardHandler {dashboard_service});
    let dashboard_handler_router = Router::new()
        .route(GET_DASHBOARD_STATS, get(get_dashboard_stats))
        .route(GET_SUCCESS_RATE, get(get_success_rate))
        .route(GET_CHART_DATA, get(get_chart_data))
        .route(GET_AVERAGE_RESPONSE_TIME, get(get_average_response_time))
        .route(GET_RECENT_ACTIVITIES, get(get_recent_activities))
        .with_state(dashboard_handler);

    Router::new()
        .merge(user_handler_router)
        .merge(swagger_router)
        .merge(auth_handler_router)
        .merge(application_handler_router)
        .merge(dashboard_handler_router)
        .layer(cors)
}
