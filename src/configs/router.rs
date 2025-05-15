use crate::configs::api_doc::ApiDoc;
use crate::configs::routes::{ADD_APPLICATION, ADD_APPLICATION_STATUS, FORGOT_PASSWORD, GET_APPLICATIONS_FOR_USER, LOGIN, RESET_PASSWORD, USER_DATA, USER_REGISTER};
use crate::handlers::application_handler::{add_application_status, fetch_applications_for_user_with_filters, register_application, ApplicationHandler};
use crate::handlers::auth_handler::{forgot_password, login, reset_password, AuthHandler};
use crate::handlers::user_handler::{get_user_data, register_user, UserHandler};
use crate::repositories::application_repository::ApplicationRepository;
use crate::repositories::user_repository::UserRepository;
use crate::services::application_service::ApplicationService;
use crate::services::auth_service::AuthService;
use crate::services::user_service::UserService;
use axum::routing::{get, post};
use axum::Router;
use dotenvy::var;
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::repositories::token_repository::TokenRepository;
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
    .with_state(auth_handler);

    let swagger_router = Router::new()
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()));


    let application_repo = ApplicationRepository::new(db_pool.clone());
    let application_service = ApplicationService::new(application_repo);
    let application_handler = Arc::new(ApplicationHandler {application_service});
    let application_handler_router = Router::new()
        .route(ADD_APPLICATION, post(register_application))
        .route(ADD_APPLICATION_STATUS, post(add_application_status))
        .route(GET_APPLICATIONS_FOR_USER, get(fetch_applications_for_user_with_filters))
        .with_state(application_handler);

    Router::new()
        .merge(user_handler_router)
        .merge(swagger_router)
        .merge(auth_handler_router)
        .merge(application_handler_router)
        .layer(cors)
}
