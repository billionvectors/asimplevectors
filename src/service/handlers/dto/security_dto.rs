// INFO: this file is not used in the project, it is just a reference for the OpenAPI documentation

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request structure for creating or updating RBAC token
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RbacTokenRequest {
    /// Space ID associated with the token
    space_id: u64,
    /// System access level (0: deny, 1: read-only, 2: write)
    system: u8,
    /// Space access level (0: deny, 1: read-only, 2: write)
    space: u8,
    /// Version access level (0: deny, 1: read-only, 2: write)
    version: u8,
    /// Vector access level (0: deny, 1: read-only, 2: write)
    vector: u8,
    /// Snapshot access level (0: deny, 1: read-only, 2: write)
    snapshot: u8,
    /// Security access level (0: deny, 1: read-only, 2: write)
    security: u8,
    /// Key-value access level (0: deny, 1: read-only, 2: write)
    keyvalue: u8,
}

/// Response structure for RBAC token creation
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RbacTokenResponse {
    /// Result of the operation
    result: String,
    /// Generated token
    token: String,
}

/// Response structure for listing RBAC tokens
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ListRbacTokensResponse {
    /// Array of token objects with detailed information
    tokens: Vec<TokenDetails>,
}

/// Structure representing detailed information of a token
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokenDetails {
    /// Unique identifier of the token
    id: u64,
    /// Space ID associated with the token
    space_id: u64,
    /// The token value
    token: String,
    /// Expiry time in UTC
    expire_time_utc: u64,
    /// System permission level
    system: u8,
    /// Space permission level
    space: u8,
    /// Version permission level
    version: u8,
    /// Vector permission level
    vector: u8,
    /// Search permission level
    search: u8,
    /// Snapshot permission level
    snapshot: u8,
    /// Security permission level
    security: u8,
    /// Key-value permission level
    keyvalue: u8,
}

/// Error response structure
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RbacTokenErrorResponse {
    /// Error message
    error: String,
}
