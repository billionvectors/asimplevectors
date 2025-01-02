// INFO: this file is not used in the project, it is just a reference for the OpenAPI documentation

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

/// VersionRequest structure for creating a new version
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionRequest {
    /// Name of the version (Required)
    name: String,
    /// Description of the version (Optional)
    #[serde(default)]
    description: Option<String>,
    /// Tag associated with the version (Optional)
    #[serde(default)]
    tag: Option<String>,
    /// Whether this version is the default version (Optional)
    #[serde(default)]
    is_default: Option<bool>,
}

/// VersionResponse structure for returning version details
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionResponse {
    /// Unique identifier for the version
    id: u64,
    /// UTC time when the version was created
    created_time_utc: u64,
    /// Description of the version
    #[serde(default)]
    description: Option<String>,
    /// Whether this version is the default version
    is_default: bool,
    /// Name of the version
    name: String,
    /// Tag associated with the version
    #[serde(default)]
    tag: Option<String>,
    /// UTC time when the version was last updated
    updated_time_utc: u64,
}

/// ErrorResponse structure for handling errors
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionErrorResponse {
    /// Error message
    error: String,
}

/// Response structure for listing all versions
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ListVersionsResponse {
    /// List of versions with their details
    values: Vec<VersionInfo>,
    total_count: usize,
}

/// VersionInfo structure containing basic version information
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionInfo {
    /// Unique identifier for the version
    id: u64,
    /// Name of the version
    name: String,
    /// Description of the version (Optional)
    #[serde(default)]
    description: Option<String>,
    /// Whether this version is the default version
    is_default: bool,
    /// Tag associated with the version (Optional)
    #[serde(default)]
    tag: Option<String>,
}