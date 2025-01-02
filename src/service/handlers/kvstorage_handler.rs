use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use async_std::fs;
use async_std::path::Path;
use tracing::{debug, error};
use rocksdb::{Options, DB, IteratorMode};

use crate::config::Config;
use crate::raft_cluster::app::App;
use crate::raft_cluster::store::Request as RaftRequest;
use crate::atinyvectors::atinyvectors_bo::ATinyVectorsBO;

use crate::service::handlers::dto::keyvalue_dto::{
    KeyValueRequest, KeyValueResponse, KeyValueErrorResponse, ListKeysResponse};

#[derive(Deserialize)]
struct QueryParams {
    start: Option<usize>,
    limit: Option<usize>,
}

// Helper function to check keyvalue permissions
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
        if bo.rbac_token.get_keyvalue_permission(&token) < 1 {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn check_write_permission(req: &Request<Arc<App>>) -> tide::Result<bool> {
    if Config::enable_security() != 0 {
        let token = extract_token(req);
        let bo = req.state().atinyvectors_bo.clone();
        if bo.rbac_token.get_keyvalue_permission(&token) < 2 {
            return Ok(false);
        }
    }
    Ok(true)
}

// POST /api/space/{space_name}/storage/{key}
#[utoipa::path(
    post,
    path = "/api/space/{space_name}/storage/{key}",
    request_body = KeyValueRequest,
    responses(
        (status = 200, description = "Key stored successfully", body = KeyValueResponse),
        (status = 403, description = "Forbidden", body = KeyValueErrorResponse)
    )
)]
pub async fn put_key(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let key = req.param("key").unwrap_or("").to_string();

    // Check if key is null or empty
    if key.is_empty() {
        return Ok(Response::builder(StatusCode::BadRequest)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&json!({"error": "Key cannot be null or empty"}))?)
            .build());
    }

    let body = req.body_string().await?;
    let wrapped_body = json!({
        "request": {
            "command": "storage_put_key",
            "space_name": space_name,
            "key": key,
            "value": body
        }
    });
    let raft_req = RaftRequest::Set {
        key: "storage_put_key".to_string(),
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

// GET /api/space/{space_name}/storage/{key}
#[utoipa::path(
    get,
    path = "/api/space/{space_name}/storage/{key}",
    responses(
        (status = 200, description = "Key retrieved successfully", body = String),
        (status = 403, description = "Forbidden", body = KeyValueErrorResponse),
        (status = 404, description = "Key not found", body = KeyValueErrorResponse)
    )
)]
pub async fn get_key(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let key = req.param("key").unwrap_or("").to_string();
    
    // Check if key is null or empty
    if key.is_empty() {
        return Ok(
            Response::builder(StatusCode::BadRequest)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Key cannot be null or empty"}))?)
                .build());
    }

    let target_directory = format!("{}/space/{}", Config::data_path(), space_name);

    // Create target directory if it does not exist
    if !std::path::Path::new(&target_directory).exists() {
        let _ = fs::create_dir_all(&target_directory).await;
    }

    let path = target_directory + "storage.rocksdb";
    if !std::path::Path::new(&path).exists() {
        tracing::debug!("No database found at path: {}", path);
        return Ok(
            Response::builder(StatusCode::BadRequest)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Key is not created or error"}))?)
                .build());
    }

    let mut db_opts = Options::default();
    db_opts.create_if_missing(true);
    let db = DB::open(&db_opts, path).unwrap();
    match db.get(key.clone())? {
        Some(v) => {
            let value = String::from_utf8(v).unwrap();
            Ok(
                Response::builder(StatusCode::Ok)
                    .header("Content-Type", "application/text")
                    .body(Body::from_string(value))
                    .build())
        },
        None => {
            Ok(
                Response::builder(StatusCode::BadRequest)
                    .header("Content-Type", "application/json")
                    .body(Body::from_json(&json!({"error": "Key is not created or error"}))?)
                    .build())
        },
    }
}

// DELETE /api/space/{space_name}/storage/{key}
#[utoipa::path(
    delete,
    path = "/api/space/{space_name}/storage/{key}",
    request_body = KeyValueRequest,
    responses(
        (status = 200, description = "Key deleted successfully", body = KeyValueResponse),
        (status = 403, description = "Forbidden", body = KeyValueErrorResponse)
    )
)]
pub async fn remove_key(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let key = req.param("key").unwrap_or("").to_string();
    
    // Check if key is null or empty
    if key.is_empty() {
        return Ok(
            Response::builder(StatusCode::BadRequest)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Key cannot be null or empty"}))?)
                .build());
    }

    let body = req.body_string().await?;
    let wrapped_body = json!({
        "request": {
            "command": "storage_remove_key",
            "space_name": space_name,
            "key": key,
            "value": body
        }
    });
    let raft_req = RaftRequest::Set {
        key: "storage_remove_key".to_string(),
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

// GET /api/space/{space_name}/keys?start={start}&limit={limit}
#[utoipa::path(
    get,
    path = "/api/space/{space_name}/keys?start={start}&limit={limit}",
    responses(
        (status = 200, description = "Keys listed successfully", body = ListKeysResponse),
        (status = 403, description = "Forbidden", body = KeyValueErrorResponse)
    )
)]
pub async fn list_keys(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let query: QueryParams = req.query()?;
    let start = query.start.unwrap_or(0);
    let limit = query.limit.unwrap_or(100);

    tracing::debug!("list_keys called with space_name: {}, start: {}, limit: {}", space_name, start, limit);

    let target_directory = format!("{}/space/{}", Config::data_path(), space_name);

    // Create target directory if it does not exist
    if !std::path::Path::new(&target_directory).exists() {
        let _ = fs::create_dir_all(&target_directory).await;
    }

    let path = target_directory + "storage.rocksdb";

    // If the database file doesn't exist, return an empty list
    if !std::path::Path::new(&path).exists() {
        return Ok(Response::builder(StatusCode::Ok)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&json!({ "keys": [], "total_count": 0 }))?)
            .build());
    }

    let mut db_opts = Options::default();
    db_opts.create_if_missing(true);
    let db = DB::open(&db_opts, path).unwrap();

    // Create a vector to hold all the keys
    let mut keys = Vec::new();

    // Get the total count of keys
    let total_count = db.iterator(IteratorMode::Start).count();

    // Iterate through the database and collect the keys with pagination
    let iter = db.iterator(IteratorMode::Start); // Iterate over the whole database
    for (i, item) in iter.enumerate().skip(start).take(limit) {
        match item {
            Ok((key, _value)) => {
                if let Ok(key_str) = String::from_utf8(key.to_vec()) {
                    keys.push(key_str);
                }
            }
            Err(e) => {
                tracing::error!("Error while iterating over keys: {}", e);
            }
        }
    }

    // Return the list of keys as a JSON response
    Ok(Response::builder(StatusCode::Ok)
        .header("Content-Type", "application/json")
        .body(Body::from_json(&json!({ "keys": keys, "total_count": total_count }))?)
        .build())
}
