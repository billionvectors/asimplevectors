use std::sync::Arc;

use openraft::error::CheckIsLeaderError;
use openraft::error::Infallible;
use tide::Body;
use tide::Request;
use tide::Response;
use tide::StatusCode;
use serde_json::Value;
use serde_json::json;

use crate::raft_atinyvectors::app::App;
use crate::raft_atinyvectors::Server;
use crate::raft_atinyvectors::TypeConfig;
use crate::raft_atinyvectors::Request as RaftRequest;
use crate::raft_atinyvectors::atinyvectors_bo::ATinyVectorsBO;

pub fn rest(app: &mut Server) {
    let mut api = app.at("/api");
    api.at("/debug/write").post(write);
    api.at("/debug/read").post(read);
    api.at("/debug/consistent_read").post(consistent_read);

    api.at("/space/:space_name").get(get_space);
    api.at("/space").post(space);
    api.at("/space/list").get(list_spaces);

    api.at("/space/:space_name/version/list").get(list_versions);
    api.at("/space/:space_name/version/:version_id").get(get_version_by_id);
    api.at("/space/:space_name/version/:version_name/by-name").get(get_version_by_name);
    api.at("/space/:space_name/version/default").get(get_default_version);
    api.at("/space/:space_name/version").post(create_version);

    api.at("/space/:space_name/vector").post(vector);
    api.at("/space/:space_name/version/:version_id/vector").post(vector_with_version);
    api.at("/space/:space_name/search").get(search);
    api.at("/space/:space_name/version/:version_id/search").get(search_with_version);
    api.at("/space/:space_name/version/:version_id/vectors").get(get_vectors_by_version_id);
}

/**
 * Application API
 *
 * This is where you place your application, you can use the example below to create your
 * API. The current implementation:
 */
async fn space(mut req: Request<Arc<App>>) -> tide::Result {
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
    let res = req.state().raft.client_write(raft_req).await;
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res)?).build())
}

// GET /api/space/{space_id}
async fn get_space(req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.get_space_by_name(&space_name);

    let res_body = match result {
        Ok(space_json) => space_json,
        Err(e) => json!({ "error": e }).to_string(),
    };

    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res_body)?).build())
}

// GET /api/space/list
async fn list_spaces(_req: Request<Arc<App>>) -> tide::Result {
    let bo = ATinyVectorsBO::new();
    let result = bo.get_space_lists();

    let res_body = match result {
        Ok(lists_json) => lists_json,
        Err(e) => json!({ "error": e }).to_string(),
    };

    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res_body)?).build())
}

// POST /space/{space_name}/version
// API to create a new version
async fn create_version(mut req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let body: Value = req.body_json().await?;

    // Create the JSON payload in the desired format
    let wrapped_body = json!({
        "request": {
            "command": "version",
            "value": body,
            "space_name": space_name
        }
    });

    // Create a Raft request to write this payload
    let raft_req = RaftRequest::Set {
        key: "version".to_string(),
        value: serde_json::to_string(&wrapped_body)?,
    };

    // Send the request to the Raft client for writing
    let res = req.state().raft.client_write(raft_req).await;
    
    // Return the response
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res)?).build())
}

// GET /space/{space_name}/version/{version_id}
async fn get_version_by_id(req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = req.param("version_id").unwrap_or("0").parse().unwrap_or(0);
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.get_version_by_id(&space_name, version_id);
    
    match result {
        Ok(version) => Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&version)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/{version_name}/by-name
async fn get_version_by_name(req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_name = req.param("version_name").unwrap_or("").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.get_version_by_name(&space_name, &version_name);

    match result {
        Ok(version) => Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&version)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/default
async fn get_default_version(req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.get_default_version(&space_name);

    match result {
        Ok(default_version) => Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&default_version)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// GET /space/{space_name}/version/list
async fn list_versions(req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.get_version_lists(&space_name);

    match result {
        Ok(versions) => Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&versions)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError).body(Body::from_string(e)).build()),
    }
}

// POST /space/{space_name}/vector
async fn vector(mut req: Request<Arc<App>>) -> tide::Result {
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
    let res = req.state().raft.client_write(raft_req).await;
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res)?).build())
}

// POST /space/{space_name}/version/{version_id}/vector
async fn vector_with_version(mut req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id = req.param("version_id").unwrap_or("default").to_string();
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
    let res = req.state().raft.client_write(raft_req).await;
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res)?).build())
}

// GET /space/{space_name}/search
async fn search(mut req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let mut body: Value = req.body_json().await?;
    body["space_name"] = Value::String(space_name);
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&body)?).build())
}

// GET /space/{space_name}/version/{version_id}/search
async fn search_with_version(mut req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id = req.param("version_id").unwrap_or("default").to_string();
    let mut body: Value = req.body_json().await?;
    body["space_name"] = Value::String(space_name);
    body["version_id"] = Value::String(version_id);
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&body)?).build())
}

// GET /space/{space_name}/version/{version_id}/vectors
async fn get_vectors_by_version_id(req: Request<Arc<App>>) -> tide::Result {
    let space_name = req.param("space_name").unwrap_or("default").to_string();
    let version_id: i32 = req.param("version_id").unwrap_or("0").parse().unwrap_or(0);
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.get_vectors_by_version_id(version_id);

    match result {
        Ok(vectors) => Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&vectors)?).build()),
        Err(e) => Ok(Response::builder(StatusCode::NotFound).body(Body::from_string(e)).build()),
    }
}

// debug api
// POST - /debug/write saves a value in a key and syncs the nodes.
// POST - /debug/read attempts to find a value from a given key.
async fn write(mut req: Request<Arc<App>>) -> tide::Result {
    let body = req.body_json().await?;
    let res = req.state().raft.client_write(body).await;
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res)?).build())
}

async fn read(mut req: Request<Arc<App>>) -> tide::Result {
    let key: String = req.body_json().await?;
    let kvs = req.state().key_values.read().await;
    let value = kvs.get(&key);

    let res: Result<String, Infallible> = Ok(value.cloned().unwrap_or_default());
    Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res)?).build())
}

async fn consistent_read(mut req: Request<Arc<App>>) -> tide::Result {
    let ret = req.state().raft.ensure_linearizable().await;

    match ret {
        Ok(_) => {
            let key: String = req.body_json().await?;
            let kvs = req.state().key_values.read().await;

            let value = kvs.get(&key);

            let res: Result<String, CheckIsLeaderError<TypeConfig>> = Ok(value.cloned().unwrap_or_default());
            Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&res)?).build())
        }
        e => Ok(Response::builder(StatusCode::Ok).body(Body::from_json(&e)?).build()),
    }
}
