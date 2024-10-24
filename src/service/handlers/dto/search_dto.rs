// INFO: this file is not used in the project, it is just a reference for the OpenAPI documentation

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request DTO for search operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SearchRequest {
    /// The vector used for searching
    vector: Vec<f32>,
}

/// Response DTO for search results
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SearchResponse {
    /// Distance between the input vector and the found vector
    distance: f64,
    /// Label corresponding to the found vector
    label: u64,
}

/// ErrorResponse DTO for search operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SearchErrorResponse {
    /// Error message
    error: String,
}
