// INFO: this file is not used in the project, it is just a reference for the OpenAPI documentation

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request structure for creating or updating vectors
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VectorRequest {
    /// List of vectors to create or update
    vectors: Vec<VectorData>,
}

/// Structure representing individual vector data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VectorData {
    /// Unique identifier for the vector
    id: u64,
    /// Vector data (array of floats)
    data: Vec<f32>,
    /// Metadata associated with the vector
    metadata: serde_json::Value,
}

/// Response structure for a successful vector operation
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VectorResponse {
    /// Result of the operation (success message)
    result: String,
}

/// Error response structure
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VectorErrorResponse {
    /// Error message
    error: String,
}

/// Response structure for retrieving vectors by version
#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetVectorsResponse {
    /// List of vectors
    vectors: Vec<VectorDataResponse>,
    total_count: usize,
}

/// Structure representing vector data in a response
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VectorDataResponse {
    /// Unique identifier for the vector
    id: u64,
    /// Vector data (array of floats)
    data: Vec<f32>,
    /// Metadata associated with the vector
    metadata: serde_json::Value,
}
