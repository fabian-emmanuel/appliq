use utoipa::{OpenApi};

/// # AppliQ API Documentation
///
/// This struct serves as the root for generating the OpenAPI (Swagger) documentation
/// for the AppliQ API. It uses `utoipa::OpenApi` to define the API's metadata,
/// paths, components (schemas), and security schemes.
///
/// The `#[openapi(...)]` macro configures various aspects of the documentation:
/// - **`info`**: General API information like title, version, description, and contact details.
/// - **`tags`**: Defines tags used to group API endpoints.
/// - **`paths`**: Lists all the API endpoint handlers that should be included in the documentation.
///   Each path points to a function in the `crate::handlers` module.
/// - **`security`**: Defines security schemes used by the API, in this case, JWT Bearer authentication.
/// - **`components(schemas(...))`**: Lists all the data structures (models, payloads, enums, errors)
///   that are part of the API, making them available in the "Schemas" section of the documentation.
///   This includes request bodies, response bodies, and other data types used by the API.
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
        (name = "AppliQ API", description = "A seamless application to help you keep track of all your job applications")
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
    ),
    security(
        ("JWT" = [])
    ),
    components(
        schemas(
            // Payloads - Auth
            crate::payloads::auth::LoginRequest,
            crate::payloads::auth::ForgotPasswordRequest,
            crate::payloads::auth::ResetPasswordRequest,

            // Payloads - User
            crate::payloads::user::UserRequest,
            crate::payloads::user::UserInfo,

            // Payloads - Application
            crate::payloads::application::ApplicationRequest,
            crate::payloads::application::ApplicationsResponse,
            crate::payloads::application::ApplicationStatusResponse,
            crate::payloads::application::ApplicationStatusRequest,
            crate::payloads::application::ApplicationFilter,
            
            // Payloads - Pagination
            crate::payloads::pagination::PaginatedResponse<crate::payloads::application::ApplicationsResponse>,

            // Models
            crate::models::user::User,
            crate::models::application::Application,
            crate::models::application::ApplicationStatus,

            // Enums
            crate::enums::roles::Role,
            crate::enums::application::ApplicationType,
            crate::enums::application::InterviewType,
            crate::enums::application::Status,
            crate::enums::application::TestType,

            // Errors
            crate::errors::api_error::ApiError
        )
    )
)]
pub struct ApiDoc;
