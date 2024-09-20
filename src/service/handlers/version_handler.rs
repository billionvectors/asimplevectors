use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use serde_json::Value;
use serde_json::json;
use crate::config::Config;
use crate::raft_cluster::app::App;
use crate::raft_cluster::store::Request as RaftRequest;
use crate::atinyvectors::atinyvectors_bo::ATinyVectorsBO;

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
// API to create a new version
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
                .body(Body::from_json(&version)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/{version_name}/by-name
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
                .body(Body::from_json(&version)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/default
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
                .body(Body::from_json(&default_version)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/list
pub async fn list_versions(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.version.get_lists(&space_name);

    match result {
        Ok(versions) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&versions)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError).body(Body::from_string(e)).build()),
    }
}
