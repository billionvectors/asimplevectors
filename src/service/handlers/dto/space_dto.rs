// INFO: this file is not used in the project, it is just a reference for the OpenAPI documentation

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

/// SpaceRequest structure for creating a new space
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct SpaceRequest {
    /// Name of the space (Required)
    name: String,
    /// Dimensionality of the space (Optional)
    #[serde(default)]
    dimension: Option<u32>,
    /// Metric type, options are "l2", "cosine", "ip" (Optional)
    #[serde(default)]
    metric: Option<String>,
    /// HNSW configuration (Optional)
    #[serde(default)]
    hnsw_config: Option<HnswConfig>,
    /// Quantization configuration (Optional)
    #[serde(default)]
    quantization_config: Option<QuantizationConfig>,
    /// Dense vector configuration (Optional)
    #[serde(default)]
    dense: Option<DenseConfig>,
    /// Sparse vector configuration (Optional)
    #[serde(default)]
    sparse: Option<SparseConfig>,
    /// Indexes configuration (Optional)
    #[serde(default)]
    indexes: Option<serde_json::Value>,
}

/// ErrorResponse structure for error messages
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct SpaceErrorResponse {
    /// Error message
    error: String,
}

/// Dense vector configuration structure
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct DenseConfig {
    /// Dimension of the dense vector (Optional)
    #[serde(default)]
    dimension: Option<u32>,
    /// Metric type for the dense vector, options are "l2", "cosine", "ip" (Optional)
    #[serde(default)]
    metric: Option<String>,
    /// HNSW configuration for dense vector (Optional)
    #[serde(default)]
    hnsw_config: Option<HnswConfig>,
    /// Quantization configuration for dense vector (Optional)
    #[serde(default)]
    quantization_config: Option<QuantizationConfig>,
}

/// Sparse vector configuration structure
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct SparseConfig {
    /// Metric type for the sparse vector, options are "l2", "cosine", "ip" (Optional)
    #[serde(default)]
    metric: Option<String>,
}

/// Space response structure
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct SpaceResponse {
    /// UTC time of creation
    created_time_utc: u64,
    /// Name of the space
    name: String,
    /// Unique identifier for the space
    spaceId: u64,
    /// UTC time of last update
    updated_time_utc: u64,
    /// Version information of the space
    version: VersionData,
}

/// Version information structure
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct VersionData {
    /// List of vector indices for the version
    vectorIndices: Vec<VectorIndexData>,
    /// Unique identifier for the version
    versionId: u64,
}

/// Vector index information structure
#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct VectorIndexData {
    /// UTC time of creation
    created_time_utc: u64,
    /// Dimensionality of the vector index
    dimension: u32,
    /// HNSW configuration for the vector index
    hnswConfig: HnswConfig,
    /// Whether this index is the default index
    is_default: bool,
    /// Metric type used in the vector index
    metricType: u8,
    /// Name of the vector index
    name: String,
    /// Quantization configuration for the vector index
    quantizationConfig: QuantizationConfig,
    /// UTC time of last update
    updated_time_utc: u64,
    /// Unique identifier for the vector index
    vectorIndexId: u64,
    /// Type of vector value
    vectorValueType: u8,
}

/// HNSW configuration structure
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct HnswConfig {
    /// EfConstruct value for HNSW configuration (Optional)
    #[serde(default)]
    EfConstruct: Option<u32>,
    /// M value for HNSW configuration (Optional)
    #[serde(default)]
    M: Option<u32>,
}

/// Quantization configuration structure
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct QuantizationConfig {
    /// Product quantization configuration (Optional)
    #[serde(default)]
    Product: Option<ProductQuantizationConfig>,
    /// Scalar quantization configuration (Optional)
    #[serde(default)]
    Scalar: Option<ScalarQuantizationConfig>,
}

/// Scalar quantization configuration structure
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ScalarQuantizationConfig {
    /// Always keep in RAM (Optional)
    #[serde(default)]
    AlwaysRam: bool,
    /// Quantile value (Optional)
    #[serde(default)]
    Quantile: f64,
    /// Quantization type, options are "f32", "f16", "int8", "int16", "int32" (Optional)
    #[serde(default)]
    Type: String,
}

/// Product quantization configuration structure
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ProductQuantizationConfig {
    /// Always keep in RAM (Optional)
    #[serde(default)]
    AlwaysRam: bool,
    /// Compression method (Optional)
    #[serde(default)]
    Compression: String,
}

/// Response structure for listing spaces
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ListSpacesResponse {
    /// List of space names and their corresponding IDs
    values: Vec<SpaceInfo>,
}

/// Space information structure
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct SpaceInfo {
    /// Name of the space
    spacename: String,
    /// ID of the space
    id: u64,
}