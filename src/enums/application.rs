use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR")]
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
pub enum InterviewType {
    Hr,
    Behavioural,
    Technical,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR")]
pub enum TestType {
    Technical,
    English,
    Aptitude,
    Other,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR")]
pub enum ApplicationType {
    Email,
    Website,
}
