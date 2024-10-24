// INFO: this file is not used in the project, it is just a reference for the OpenAPI documentation

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request DTO for key-value operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct KeyValueRequest {
    /// put the storage operation
    text: String,
}

/// Response DTO for key-value operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct KeyValueResponse {
    /// Result of the operation, should be "success" for successful operations
    result: String,
}

/// ErrorResponse DTO for key-value operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct KeyValueErrorResponse {
    /// Error message
    error: String,
}

/// List of keys response DTO
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ListKeysResponse {
    /// List of keys stored in the system
    keys: Vec<String>,
}
