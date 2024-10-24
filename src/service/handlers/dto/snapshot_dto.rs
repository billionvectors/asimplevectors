// INFO: this file is not used in the project, it is just a reference for the OpenAPI documentation

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request DTO for creating a snapshot
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateSnapshotRequest {
    /// The name of the space for which the snapshot is created
    spacename: String,
}

/// Response DTO for snapshot operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SnapshotResponse {
    /// Operation result message
    result: String,
}

/// ErrorResponse DTO for snapshot operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SnapshotErrorResponse {
    /// Error message
    error: String,
}

/// DTO for listing snapshots
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ListSnapshotsResponse {
    /// List of snapshots available
    snapshots: Vec<SnapshotInfo>,
}

/// DTO for snapshot information
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SnapshotInfo {
    /// Name of the snapshot file
    file_name: String,
}
