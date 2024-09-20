use std::sync::Arc;
use tide::{Body, Request, Response, StatusCode};
use serde_json::Value;
use crate::config::Config;
use crate::raft_cluster::app::App;

// POST /api/space/{space_name}/search
pub async fn search(mut req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id = 0;
    let body: Value = req.body_json().await?;
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.search.search(&space_name, version_id, &body.to_string(), 10);

    match result {
        Ok(versions) => {
            Ok(Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&versions)?)
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
pub async fn search_with_version(mut req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = req.param("version_id").unwrap_or("0").parse().unwrap_or(0);
    let body: Value = req.body_json().await?;
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.search.search(&space_name, version_id, &body.to_string(), 10);

    match result {
        Ok(versions) => {
            Ok(Response::builder(StatusCode::Ok)
                .header("Content-Type", "application/json")
                .body(Body::from_json(&versions)?)
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
