use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request DTO for rerank operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RerankRequest {
    /// The space name where the rerank operation is performed
    space_name: String,
    /// The version unique ID associated with the rerank operation
    version_unique_id: i32,
    /// The initial search results as an array of vector IDs
    vector_ids: Vec<u64>,
    /// Query terms for BM25-based reranking
    query_terms: Vec<String>,
    /// Top K results to return after reranking
    top_k: usize,
}

/// Response DTO for rerank results
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RerankResponse {
    /// The vector unique ID after reranking
    id: u64,
    /// Distance of the vector from the query
    distance: f64,
    /// The BM25 score of the vector
    bm25_score: f64,
}

/// ErrorResponse DTO for rerank operations
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RerankErrorResponse {
    /// Error message
    error: String,
}
