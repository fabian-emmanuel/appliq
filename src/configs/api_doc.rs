use utoipa::{OpenApi};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "AppliQ API",
        version = "1.0.0",
        description = "A seamless application to help you keep track of all your job applications.",
        contact(
            name = "Fabian Emmanuel",
            email = "emmanuel.fabian.dev@gmail.com",
            url = "https://yourwebsite.com"
        )
    ),
    tags(
        (name = "AppliQ API", description = "A seamless application to help you keep track of all your applications")
    ),
    paths(
        crate::handlers::user_handler::register_user,
        crate::handlers::user_handler::get_user_data,
        crate::handlers::auth_handler::login,
        crate::handlers::auth_handler::forgot_password,
        crate::handlers::auth_handler::reset_password,
        crate::handlers::application_handler::register_application,
        crate::handlers::application_handler::add_application_status,
        crate::handlers::application_handler::fetch_applications_for_user_with_filters,
        crate::handlers::dashboard_handler::get_dashboard_stats,
        crate::handlers::dashboard_handler::get_success_rate,
        crate::handlers::dashboard_handler::get_chart_data,
    ),
    security(
        ("JWT" = [])
    )
)]
pub struct ApiDoc;
