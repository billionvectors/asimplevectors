use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use serde_json::Value;
use serde_json::json;
use tracing::debug;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

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
        let permission = bo.rbac_token.get_space_permission(&token);
        debug!("token={} permission={}", token, permission);

        if permission < 1 {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn check_write_permission(req: &Request<Arc<App>>) -> tide::Result<bool> {
    if Config::enable_security() != 0 {
        debug!("check_write_permission");
        let token = extract_token(req);
        let bo = req.state().atinyvectors_bo.clone();
        let permission = bo.rbac_token.get_space_permission(&token);
        debug!("token={} permission={}", token, permission);

        if permission < 2 {
            return Ok(false);
        }
    }
    Ok(true)
}

// POST /api/space
#[utoipa::path(
    post,
    path = "/api/space",
    responses(
        (status = 200, description = "Create spaces successfully")
    )
)]
pub async fn space(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let body: Value = req.body_json().await?;
    let wrapped_body = json!({
        "request": {
            "command": "space",
            "value": body
        }
    });
    let raft_req = RaftRequest::Set {
        key: "space".to_string(),
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

// GET /api/space/{space_name}
#[utoipa::path(
    get,
    path = "/api/space/{space_name}",
    responses(
        (status = 200, description = "Get Space")
    )
)]
pub async fn get_space(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.space.get_by_space_name(&space_name);

    let res_body = match result {
        Ok(space_json) => space_json,
        Err(e) => json!({ "error": e }).to_string(),
    };

    Ok(
        Response::builder(StatusCode::Ok)
            .header("Content-Type", "application/json")
            .body(Body::from_string(res_body)).build())
}

// DELETE /api/space/{space_name}
#[utoipa::path(
    delete,
    path = "/api/space/{space_name}",
    responses(
        (status = 200, description = "Delete Space successfuly")
    )
)]
pub async fn delete_space(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let body: Value = req.body_json().await?;
    let wrapped_body = json!({
        "request": {
            "command": "delete_space",
            "value": body
        }
    });
    let raft_req = RaftRequest::Set {
        key: "delete_space".to_string(),
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

// GET /api/space/list
#[utoipa::path(
    delete,
    path = "/api/space/list",
    responses(
        (status = 200, description = "List Space successfuly")
    )
)]
pub async fn list_spaces(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let bo = ATinyVectorsBO::new();
    let result = bo.space.get_lists();

    let res_body = match result {
        Ok(lists_json) => lists_json,
        Err(e) => json!({ "error": e }).to_string(),
    };

    Ok(
        Response::builder(StatusCode::Ok)
            .header("Content-Type", "application/json")
            .body(Body::from_string(res_body)).build())
}
