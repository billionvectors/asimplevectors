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

use crate::service::handlers::dto::version_dto::{
    VersionRequest, VersionResponse, VersionErrorResponse, ListVersionsResponse, VersionInfo};

#[derive(Deserialize)]
struct QueryParams {
    start: Option<usize>,
    limit: Option<usize>,
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
        if bo.rbac_token.get_version_permission(&token) < 1 {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn check_write_permission(req: &Request<Arc<App>>) -> tide::Result<bool> {
    if Config::enable_security() != 0 {
        let token = extract_token(req);
        let bo = req.state().atinyvectors_bo.clone();
        if bo.rbac_token.get_version_permission(&token) < 2 {
            return Ok(false);
        }
    }
    Ok(true)
}

// POST /space/{space_name}/version
#[utoipa::path(
    post,
    path = "/space/{space_name}/version",
    request_body = VersionRequest,
    responses(
        (status = 200, description = "Version created successfully", body = VersionResponse),
        (status = 403, description = "Forbidden", body = VersionErrorResponse)
    )
)]
pub async fn create_version(mut req: Request<Arc<App>>) -> tide::Result {
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
            "command": "version",
            "value": body,
            "space_name": space_name
        }
    });

    let raft_req = RaftRequest::Set {
        key: "version".to_string(),
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

// GET /space/{space_name}/version/{version_id}
#[utoipa::path(
    get,
    path = "/space/{space_name}/version/{version_id}",
    responses(
        (status = 200, description = "Version details retrieved successfully", body = VersionResponse),
        (status = 403, description = "Forbidden", body = VersionErrorResponse),
        (status = 404, description = "Version not found", body = VersionErrorResponse)
    )
)]
pub async fn get_version_by_id(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = req.param("version_id").unwrap_or("0").parse().unwrap_or(0);
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.version.get_by_version_id(&space_name, version_id);
    
    match result {
        Ok(version) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(version)).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/{version_name}/by-name
#[utoipa::path(
    get,
    path = "/space/{space_name}/version/{version_name}/by-name",
    responses(
        (status = 200, description = "Version details retrieved successfully", body = VersionResponse),
        (status = 403, description = "Forbidden", body = VersionErrorResponse),
        (status = 404, description = "Version not found", body = VersionErrorResponse)
    )
)]
pub async fn get_version_by_name(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_name = req.param("version_name").unwrap_or("").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.version.get_by_version_name(&space_name, &version_name);

    match result {
        Ok(version) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(version)).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/default
#[utoipa::path(
    get,
    path = "/space/{space_name}/version/default",
    responses(
        (status = 200, description = "Version details retrieved successfully", body = VersionResponse),
        (status = 403, description = "Forbidden", body = VersionErrorResponse),
        (status = 404, description = "Version not found", body = VersionErrorResponse)
    )
)]
pub async fn get_default_version(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.version.get_default_version(&space_name);

    match result {
        Ok(default_version) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(default_version)).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/versions?start={start}&limit={limit}
#[utoipa::path(
    get,
    path = "/space/{space_name}/versions?start={start}&limit={limit}",
    responses(
        (status = 200, description = "Versions listed successfully", body = ListVersionsResponse),
        (status = 403, description = "Forbidden", body = VersionErrorResponse)
    )
)]
pub async fn list_versions(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }


    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let query: QueryParams = req.query()?;
    let start = query.start.unwrap_or(0) as i32;
    let limit = query.limit.unwrap_or(100) as i32;

    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.version.get_lists(&space_name, start, limit);

    match result {
        Ok(versions) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(versions)).build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError).body(Body::from_string(e)).build()),
    }
}

// DELETE /space/{space_name}/version/{version_id}
#[utoipa::path(
    delete,
    path = "/space/{space_name}/version/{version_id}",
    responses(
        (status = 200, description = "Version deleted successfully", body = VersionResponse),
        (status = 403, description = "Forbidden", body = VersionErrorResponse),
        (status = 404, description = "Version not found", body = VersionErrorResponse)
    )
)]
pub async fn delete_version(req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = req.param("version_id").unwrap_or("0").parse().unwrap_or(0);

    let wrapped_body = json!({
        "request": {
            "command": "delete_version",
            "version_id": version_id,
            "space_name": space_name
        }
    });

    let raft_req = RaftRequest::Set {
        key: "delete_version".to_string(),
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
