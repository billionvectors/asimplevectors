use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::config::Config;
use crate::raft_cluster::app::App;
use crate::raft_cluster::store::Request as RaftRequest;
use crate::atinyvectors::atinyvectors_bo::ATinyVectorsBO;

use crate::service::handlers::dto::vector_dto::{
    VectorData, VectorRequest, VectorResponse, VectorErrorResponse, GetVectorsResponse, VectorDataResponse};

#[derive(Deserialize)]
struct QueryParams {
    start: Option<usize>,
    limit: Option<usize>,
    filter: Option<String>,
}
    
// Helper function to check snapshot permissions
fn extract_token(req: &Request<Arc<App>>) -> String {
    req.header("Authorization")
        .and_then(|header| header.get(0))
        .map(|header_value| header_value.as_str().trim_start_matches("Bearer ").to_string())
        .unwrap_or_default()
}

async fn check_read_permission(req: &Request<Arc<App>>) -> tide::Result<bool> {
    if Config::enable_security() != 0 {
        let token = extract_token(req);
        let bo = req.state().atinyvectors_bo.clone();
        if bo.rbac_token.get_vector_permission(&token) < 1 {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn check_write_permission(req: &Request<Arc<App>>) -> tide::Result<bool> {
    if Config::enable_security() != 0 {
        let token = extract_token(req);
        let bo = req.state().atinyvectors_bo.clone();
        if bo.rbac_token.get_vector_permission(&token) < 2 {
            return Ok(false);
        }
    }
    Ok(true)
}

// POST /space/{space_name}/vector
#[utoipa::path(
    post,
    path = "/space/{space_name}/vector",
    request_body = VectorRequest,
    responses(
        (status = 200, description = "Vector created successfully", body = VectorResponse),
        (status = 403, description = "Forbidden", body = VectorErrorResponse)
    )
)]
pub async fn vector(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();

    let body: Value = req.body_json().await?;
    let wrapped_body = json!({
        "request": {
            "command": "vector",
            "space_name": space_name,
            "value": body
        }
    });
    let raft_req = RaftRequest::Set {
        key: "vector".to_string(),
        value: serde_json::to_string(&wrapped_body)?,
    };

    // Send a write request to the Raft client
    let res = req.state().raft.client_write(raft_req).await;

    // Handle response
    match res {
        Ok(_) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"result": "success"}))?)
                .build()),
        Err(e) => Ok(
            Response::builder(StatusCode::InternalServerError)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": e.to_string()}))?)
                .build()),
    }
}

// POST /space/{space_name}/version/{version_id}/vector
#[utoipa::path(
    post,
    path = "/space/{space_name}/version/{version_id}/vector",
    request_body = VectorRequest,
    responses(
        (status = 200, description = "Vector added to version successfully", body = VectorResponse),
        (status = 403, description = "Forbidden", body = VectorErrorResponse)
    )
)]
pub async fn vector_with_version(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id = req.param("version_id").unwrap_or("0").to_string();

    let body: Value = req.body_json().await?;
    let wrapped_body = json!({
        "request": {
            "command": "vector_with_version",
            "space_name": space_name,
            "version_id": version_id,
            "value": body
        }
    });
    let raft_req = RaftRequest::Set {
        key: "vector_with_version".to_string(),
        value: serde_json::to_string(&wrapped_body)?,
    };

    // Send a write request to the Raft client
    let res = req.state().raft.client_write(raft_req).await;

    // Handle response
    match res {
        Ok(_) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"result": "success"}))?)
                .build()),
        Err(e) => Ok(
            Response::builder(StatusCode::InternalServerError)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": e.to_string()}))?)
                .build()),
    }
}

// GET /space/{space_name}/version/{version_id}/vectors?start=0&limit=10&filter=
#[utoipa::path(
    get,
    path = "/space/{space_name}/version/{version_id}/vectors",
    params(
        ("start" = i32, Path, description = "Starting index of vectors"),
        ("limit" = i32, Path, description = "Maximum number of vectors to retrieve"),
        ("filter" = String, Query, description = "Filter to apply on vectors")
    ),
    responses(
        (status = 200, description = "Vectors retrieved successfully", body = GetVectorsResponse),
        (status = 403, description = "Forbidden", body = VectorErrorResponse),
        (status = 404, description = "Vectors not found", body = VectorErrorResponse)
    )
)]
pub async fn get_vectors_by_version_id(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = req.param("version_id").unwrap_or("0").parse().unwrap_or(0);
    let start: i32 = req.param("start").unwrap_or("0").parse().unwrap_or(0);
    let limit: i32 = req.param("limit").unwrap_or("10").parse().unwrap_or(0);
    let filter = req.query::<QueryParams>()?.filter.unwrap_or_default();

    let bo = req.state().atinyvectors_bo.clone();

    let result = bo.vector.get_vectors_by_version_id(
        space_name.as_str(), version_id, start, limit, filter.as_str());

    match result {
        Ok(vectors) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(vectors)).build()),
        Err(e) => Ok(
            Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/vectors?start=0&limit=10&filter=
#[utoipa::path(
    get,
    path = "/space/{space_name}/vectors",
    params(
        ("start" = i32, Path, description = "Starting index of vectors"),
        ("limit" = i32, Path, description = "Maximum number of vectors to retrieve"),
        ("filter" = String, Query, description = "Filter to apply on vectors")
    ),
    responses(
        (status = 200, description = "Vectors retrieved successfully", body = GetVectorsResponse),
        (status = 403, description = "Forbidden", body = VectorErrorResponse),
        (status = 404, description = "Vectors not found", body = VectorErrorResponse)
    )
)]
pub async fn get_vectors_by_default_version(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = 0; // default unique version
    let query: QueryParams = req.query()?;
    let start: i32 = query.start.unwrap_or(0) as i32;
    let limit: i32 = query.limit.unwrap_or(10) as i32;
    let filter = query.filter.unwrap_or_default();

    let bo = req.state().atinyvectors_bo.clone();

    let result = bo.vector.get_vectors_by_version_id(
        space_name.as_str(), version_id, start, limit, filter.as_str());

    match result {
        Ok(vectors) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(vectors)).build()),
        Err(e) => Ok(
            Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}