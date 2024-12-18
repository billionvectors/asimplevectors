use std::borrow::Borrow;
use std::sync::Arc;

use async_std::fs;
use async_std::io::WriteExt;
use async_std::path::Path;
use async_std::path::PathBuf;

use tide::{Body, Request, Response, StatusCode};
use multer::{Multipart, bytes::Bytes};
use serde_json::Value;
use serde_json::json;
use futures::stream;
use regex::Regex;
use chrono::Local;

use crate::config::Config;
use crate::raft_cluster::app::App;
use crate::raft_cluster::store::Request as RaftRequest;
use crate::atinyvectors::atinyvectors_bo::ATinyVectorsBO;

use crate::service::handlers::dto::snapshot_dto::{
    CreateSnapshotRequest, SnapshotResponse, SnapshotErrorResponse, ListSnapshotsResponse, SnapshotInfo};

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
        if bo.rbac_token.get_snapshot_permission(&token) < 1 {
            return Ok(false);
        }
    }
    Ok(true)
}

async fn check_write_permission(req: &Request<Arc<App>>) -> tide::Result<bool> {
    if Config::enable_security() != 0 {
        let token = extract_token(req);
        let bo = req.state().atinyvectors_bo.clone();
        if bo.rbac_token.get_snapshot_permission(&token) < 2 {
            return Ok(false);
        }
    }
    Ok(true)
}

// POST /snapshot
#[utoipa::path(
    post,
    path = "/snapshot",
    request_body = CreateSnapshotRequest,
    responses(
        (status = 200, description = "Snapshot created successfully", body = SnapshotResponse),
        (status = 403, description = "Forbidden", body = SnapshotErrorResponse)
    )
)]
pub async fn create_snapshot(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(Response::builder(StatusCode::Forbidden)
            .body(Body::from_json(&json!({"error": "Forbidden"}))?)
            .build());
    }

    let body: Value = req.body_json().await?;
    let wrapped_body = json!({
        "request": {
            "command": "create_snapshot",
            "value": body
        }
    });
    let raft_req = RaftRequest::Set {
        key: "create_snapshot".to_string(),
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

// POST /snapshot/{file_name}/restore
#[utoipa::path(
    post,
    path = "/snapshot/{file_name}/restore",
    responses(
        (status = 200, description = "Snapshot restored successfully", body = SnapshotResponse),
        (status = 403, description = "Forbidden", body = SnapshotErrorResponse)
    )
)]
pub async fn restore_snapshot(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let file_name = req.param("file_name").unwrap_or("default").to_string();
    let file_name = format!("snapshot-{}.zip", file_name);
    tracing::info!("restore_snapshot: file_name={}", file_name);

    let body: Value = match req.body_json().await {
        Ok(json) => json,
        Err(_) => json!({}),
    };

    let wrapped_body = json!({
        "request": {
            "command": "snapshot_restore",
            "value": body,
            "file_name": file_name,
        }
    });

    let raft_req = RaftRequest::Set {
        key: "snapshot_restore".to_string(),
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

// DELETE /snapshot/{file_name}/delete
#[utoipa::path(
    delete,
    path = "/snapshot/{file_name}/delete",
    responses(
        (status = 200, description = "Snapshots deleted successfully", body = SnapshotResponse),
        (status = 403, description = "Forbidden", body = SnapshotErrorResponse)
    )
)]
pub async fn delete_snapshot(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let file_name = req.param("file_name").unwrap_or("default").to_string();
    let file_name = format!("snapshot-{}.zip", file_name);
    tracing::info!("delete_snapshot: file_name={}", file_name);
    
    let wrapped_body = json!({
        "request": {
            "command": "snapshot_delete",
            "file_name": file_name,
        }
    });

    let raft_req = RaftRequest::Set {
        key: "snapshot_delete".to_string(),
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

// GET /snapshots
#[utoipa::path(
    get,
    path = "/snapshots",
    responses(
        (status = 200, description = "List of snapshots retrieved successfully", body = ListSnapshotsResponse),
        (status = 403, description = "Forbidden", body = SnapshotErrorResponse)
    )
)]
pub async fn list_snapshots(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.snapshot.list_snapshots();

    match result {
        Ok(snapshots) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_string(snapshots)).build()),
        Err(e) => Ok(
            Response::builder(StatusCode::InternalServerError)
                .body(Body::from_string(e)).build()),
    }
}

// GET /snapshot/{file_name}/download
#[utoipa::path(
    get,
    path = "/snapshot/{file_name}/download",
    responses(
        (status = 200, description = "Snapshot downloaded successfully", body = String),
        (status = 403, description = "Forbidden", body = SnapshotErrorResponse)
    )
)]
pub async fn download_snapshot(req: Request<Arc<App>>) -> tide::Result {
    if !check_read_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let file_name = req.param("file_name").unwrap_or("default").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let file_name = format!("snapshot-{}.zip", file_name);
    tracing::info!("download_snapshot: file_name={}", file_name);

    match bo.snapshot.download_snapshot(file_name.as_str()).await {
        Ok(snapshot_path) => {
            tracing::debug!("download_snapshot: snapshot_path={}", snapshot_path.display());
            let file_body = Body::from_file(&snapshot_path).await.map_err(|e| {
                tide::Error::from_str(StatusCode::InternalServerError, format!("Failed to read file: {}", e))
            })?;

            let file_name = snapshot_path.file_name().and_then(|n| n.to_str()).unwrap_or("snapshot.zip").to_string();

            let mut res = Response::new(StatusCode::Ok);
            res.set_body(file_body);
            res.insert_header("Content-Disposition", format!("attachment; filename=\"{}\"", file_name));
            res.insert_header("Content-Type", "application/zip");

            Ok(res)
        }
        Err(e) => Ok(
            Response::builder(StatusCode::InternalServerError)
                .header("Content-Type", "application/json")
                .body(json!({"error": e}))
                .build()),
    }
}

// POST /snapshots/restore
pub async fn restore_snapshot_from_upload(mut req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    tracing::info!("restore_snapshot_from_upload");

    // Extract the Content-Type header
    let content_type = match req.header("Content-Type") {
        Some(cts) => {
            match cts.get(0) {
                Some(hv) => hv.to_string(),
                None => {
                    tracing::debug!("Content-Type is missing");
                    return Ok(
                        Response::builder(StatusCode::BadRequest)
                            .header("Content-Type", "application/json")
                            .body(json!({"error": "Invalid Content-Type header"}))
                            .build());
                }
            }
        },
        None => {
            tracing::debug!("Content-Type is missing");
            return Ok(
                Response::builder(StatusCode::BadRequest)
                    .header("Content-Type", "application/json")
                    .body(json!({"error": "Missing Content-Type header"}))
                    .build());
        }
    };

    // Extract boundary from Content-Type
    tracing::debug!("content_type={:?}", content_type);
    let boundary = match multer::parse_boundary(&content_type) {
        Ok(b) => b,
        Err(_) => {
            tracing::debug!("Extract boundary from Content-Type failed");
            return Ok(
                Response::builder(StatusCode::BadRequest)
                    .header("Content-Type", "application/json")
                    .body(json!({"error": "Invalid Content-Type header"}))
                    .build());
        }
    };

    // Collect the entire body as bytes
    let body_bytes = match req.body_bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::debug!("Collect the entire body as bytes failed");
            return Ok(
                Response::builder(StatusCode::BadRequest)
                    .header("Content-Type", "application/json")
                    .body(json!({"error": format!("Failed to read request body: {}", e)}))
                    .build());
        }
    };

    // Convert body_bytes to multer::bytes::Bytes
    let body_stream = stream::once(async move { 
        Ok::<Bytes, std::io::Error>(Bytes::from(body_bytes)) 
    });

    // Initialize multer Multipart parser
    tracing::debug!("Initialize multer Multipart parser");
    let mut multipart = Multipart::new(body_stream, boundary);

    // Variables to store file path and original file name
    let mut file_path: Option<PathBuf> = None;
    let mut original_file_name: Option<String> = None;

    let defaultFileName = {
        let date = Local::now().format("%Y%m%d%H%M").to_string();
        format!("snapshot-{}.zip", date)
    };

    // Iterate through multipart fields
    while let Some(mut field) = multipart.next_field().await? {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "file" {
            let filename = match field.file_name() {
                Some(name) => name.to_string(),
                None => defaultFileName.clone(),
            };
            original_file_name = Some(filename.clone());
            tracing::debug!("filename: {}", filename);

            let data_path = Config::data_path().clone();
            let snapshot_dir = Path::new(&data_path);
            let temp_dir = snapshot_dir.join("temp");

            if !temp_dir.exists().await {
                fs::create_dir_all(&temp_dir).await.map_err(|e| {
                    tide::Error::from_str(
                        StatusCode::InternalServerError,
                        format!("Failed to create temp directory: {}", e),
                    )
                })?;
            }

            let filepath = temp_dir.join(&filename);
            let mut file = fs::File::create(&filepath).await.map_err(|e| {
                tide::Error::from_str(
                    StatusCode::InternalServerError,
                    format!("Failed to create file: {}", e),
                )
            })?;

            while let Some(chunk) = field.chunk().await? {
                file.write_all(&chunk).await.map_err(|e| {
                    tide::Error::from_str(
                        StatusCode::InternalServerError,
                        format!("Failed to write file: {}", e),
                    )
                })?;
            }

            file_path = Some(filepath);
        }
    }

    tracing::debug!("file_path: {:?}", file_path);

    let file_path = match file_path {
        Some(path) => path,
        None => {
            tracing::debug!("file does not exists!");
            return Ok(
                Response::builder(StatusCode::BadRequest)
                    .header("Content-Type", "application/json")
                    .body(json!({"error": "No file field in multipart"}))
                    .build());
        }
    };

    let original_file_name = match original_file_name {
        Some(name) => name,
        None => defaultFileName.clone(),
    };

    let re = Regex::new(r"^snapshot-(?P<date>\d{12})\.zip$").map_err(|e| {
        tide::Error::from_str(
            StatusCode::InternalServerError,
            format!("Failed to compile regex: {}", e),
        )
    })?;

    let caps = match re.captures(&original_file_name) {
        Some(c) => c,
        None => {
            return Ok(
                Response::builder(StatusCode::BadRequest)
                    .header("Content-Type", "application/json")
                    .body(json!({"error": 
                        format!("Invalid filename format: input={} / fileformat=snapshot-yyyymmddHHMM.zip", original_file_name)}))
                    .build());
        }
    };

    let snapshot_dir = PathBuf::from(Config::data_path()).join("snapshot");
    tracing::debug!("Snapshot directory path: {:?}", snapshot_dir);

    if !snapshot_dir.exists().await {
        tracing::debug!("Snapshot directory does not exist. Creating...");
        fs::create_dir_all(&snapshot_dir).await.map_err(|e| {
            tracing::error!("Failed to create snapshot directory: {}", e);
            tide::Error::from_str(
                StatusCode::InternalServerError,
                format!("Failed to create snapshot directory: {}", e),
            )
        })?;
        tracing::debug!("Snapshot directory created successfully");
    }

    let target_path = snapshot_dir.join(&original_file_name);
    tracing::debug!("Target snapshot file path: {:?}", target_path);

    tracing::debug!("Copying file from {:?} to {:?}", file_path, target_path);
    fs::copy(&file_path, &target_path).await.map_err(|e| {
        tracing::error!("Failed to copy snapshot file: {}", e);
        tide::Error::from_str(
            StatusCode::InternalServerError,
            format!("Failed to copy snapshot file: {}", e),
        )
    })?;
    tracing::debug!("File copied successfully");

    fs::remove_file(&file_path).await.map_err(|e| {
        tracing::error!("Failed to remove temp file: {}", e);
        tide::Error::from_str(
            StatusCode::InternalServerError,
            format!("Failed to remove temp file: {}", e),
        )
    })?;

    let file_name = original_file_name.clone();
    tracing::info!("restore_snapshot: file_name={}", file_name);

    let leader_id = Config::instance_id();
    let leader_addr = Config::http_addr();
    
    let wrapped_body = json!({
        "request": {
            "command": "snapshot_sync",
            "value": "{}", // current empty value
            "file_name": file_name,
            "leader_id": leader_id,
            "leader_addr": leader_addr,
        }
    });

    let raft_req = RaftRequest::Set {
        key: "snapshot_sync".to_string(),
        value: serde_json::to_string(&wrapped_body)?,
    };

    tracing::debug!("raft client write");

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

// DELETE /snapshots/delete_all
#[utoipa::path(
    delete,
    path = "/snapshots/delete_all",
    responses(
        (status = 200, description = "Snapshots deleted successfully", body = SnapshotResponse),
        (status = 403, description = "Forbidden", body = SnapshotErrorResponse)
    )
)]
pub async fn delete_all_snapshots(req: Request<Arc<App>>) -> tide::Result {
    if !check_write_permission(&req).await? {
        return Ok(
            Response::builder(StatusCode::Forbidden)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&json!({"error": "Forbidden"}))?)
                .build());
    }

    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.snapshot.delete_snapshots();

    match result {
        Ok(_) => Ok(
            Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(json!({"status": "Snapshots deleted successfully"})).build()),
        Err(e) => Ok(
            Response::builder(StatusCode::InternalServerError)
                .header("Content-Type", "application/json")
                .body(json!({"error": e})).build()),
    }
}