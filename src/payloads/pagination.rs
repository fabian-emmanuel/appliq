use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(description = "Provides a paginated list of items.")]
pub struct PaginatedResponse<T: utoipa::ToSchema + 'static> {
    #[schema(description = "List of items on the current page.")]
    pub items: Vec<T>,
    #[schema(description = "Total number of items across all pages.", example = 100)]
    pub total_items: i64,
    #[schema(description = "Current page number.", example = 1)]
    pub page: i64,
    #[schema(description = "Number of items per page.", example = 10)]
    pub page_size: i64,
    #[schema(description = "Total number of pages.", example = 10)]
    pub total_pages: i64,
}
