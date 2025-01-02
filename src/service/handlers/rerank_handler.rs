use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use serde_json::Value;
use serde_json::json;
use crate::config::Config;
use crate::raft_cluster::app::App;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::service::handlers::dto::rerank_dto::{
    RerankRequest, RerankResponse, RerankErrorResponse
};

// Helper function to check rerank permissions
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
        if bo.rbac_token.get_search_permission(&token) < 1 {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn check_write_permission(req: &Request<Arc<App>>) -> tide::Result<bool> {
    if Config::enable_security() != 0 {
        let token = extract_token(req);
        let bo = req.state().atinyvectors_bo.clone();
        if bo.rbac_token.get_search_permission(&token) < 2 {
            return Ok(false);
        }
    }
    Ok(true)
}

// POST /api/space/{space_name}/rerank
#[utoipa::path(
    post,
    path = "/api/space/{space_name}/rerank",
    request_body = RerankRequest,
    responses(
        (status = 200, description = "Rerank results successfully retrieved", body = [RerankResponse]),
        (status = 403, description = "Forbidden", body = RerankErrorResponse)
    )
)]
pub async fn rerank(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id = 0;

    let body: Value = req.body_json().await?;
    let k = if let Some(top_k) = body.get("top_k").and_then(|v| v.as_u64()) {
        top_k as usize
    } else if let Some(k_value) = body.get("k").and_then(|v| v.as_u64()) {
        k_value as usize
    } else {
        10
    };

    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.rerank.rerank(&space_name, version_id, &body.to_string(), k);

    match result {
        Ok(versions) => {
            Ok(Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(versions))
                .build())
        },
        Err(e) => {
            Ok(Response::builder(StatusCode::InternalServerError)
                .header("Content-Type", "application/text")
                .body(Body::from_string(e))
                .build())
        }
    }
}

// POST /api/space/{space_name}/version/{version_id}/rerank
#[utoipa::path(
    post,
    path = "/api/space/{space_name}/version/{version_id}/rerank",
    request_body = RerankRequest,
    responses(
        (status = 200, description = "Rerank results successfully retrieved", body = [RerankResponse]),
        (status = 403, description = "Forbidden", body = RerankErrorResponse)
    )
)]
pub async fn rerank_with_version(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = req.param("version_id").unwrap_or("0").parse().unwrap_or(0);

    let body: Value = req.body_json().await?;
    let k = if let Some(top_k) = body.get("top_k").and_then(|v| v.as_u64()) {
        top_k as usize
    } else if let Some(k_value) = body.get("k").and_then(|v| v.as_u64()) {
        k_value as usize
    } else {
        10
    };

    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.rerank.rerank(&space_name, version_id, &body.to_string(), k);

    match result {
        Ok(versions) => {
            Ok(Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(versions))
                .build())
        },
        Err(e) => {
            Ok(Response::builder(StatusCode::InternalServerError)
                .header("Content-Type", "application/text")
                .body(Body::from_string(e))
                .build())
        }
    }
}
