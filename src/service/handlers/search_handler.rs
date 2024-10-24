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

use crate::service::handlers::dto::search_dto::{
    SearchRequest, SearchResponse, SearchErrorResponse
};

// Helper function to check search permissions
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

// POST /api/space/{space_name}/search
#[utoipa::path(
    post,
    path = "/api/space/{space_name}/search",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Search results successfully retrieved", body = [SearchResponse]),
        (status = 403, description = "Forbidden", body = SearchErrorResponse)
    )
)]
pub async fn search(mut req: Request<Arc<App>>) -> tide::Result {
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
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.search.search(&space_name, version_id, &body.to_string(), 10);

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

// POST /api/space/{space_name}/version/{version_id}/search
#[utoipa::path(
    post,
    path = "/api/space/{space_name}/version/{version_id}/search",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Search results successfully retrieved", body = [SearchResponse]),
        (status = 403, description = "Forbidden", body = SearchErrorResponse)
    )
)]
pub async fn search_with_version(mut req: Request<Arc<App>>) -> tide::Result {
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
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.search.search(&space_name, version_id, &body.to_string(), 10);

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
