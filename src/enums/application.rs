use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR")]
#[schema(description = "Defines the current status of a job application.")]
pub enum Status {
    Applied,
    Test,
    Interview,
    OfferAwarded,
    Rejected,
    Withdrawn,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR")]
#[schema(description = "Defines the type of interview conducted.")]
pub enum InterviewType {
    Hr,
    Behavioural,
    Technical,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR")]
#[schema(description = "Defines the type of test administered.")]
pub enum TestType {
    Technical,
    English,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR")]
#[schema(description = "Defines how the application was submitted.")]
pub enum ApplicationType {
    Email,
    Website,
}
