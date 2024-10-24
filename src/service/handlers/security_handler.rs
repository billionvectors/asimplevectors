use std::sync::Arc;

use tide::{Body, Request, Response, StatusCode};
use serde_json::Value;
use serde_json::json;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::config::Config;
use crate::raft_cluster::app::App;
use crate::raft_cluster::store::Request as RaftRequest;

use crate::service::handlers::dto::security_dto::{
    RbacTokenRequest, RbacTokenResponse, RbacTokenErrorResponse, ListRbacTokensResponse, TokenDetails};

// security api should be access control by ip address

// POST /api/security/tokens
#[utoipa::path(
    post,
    path = "/api/security/tokens",
    request_body = RbacTokenRequest,
    responses(
        (status = 201, description = "RBAC token created successfully", body = RbacTokenResponse),
        (status = 500, description = "Internal Server Error", body = RbacTokenErrorResponse)
    )
)]
pub async fn create_rbac_token(mut req: Request<Arc<App>>) -> tide::Result {
    // Parse the JSON body from the request
    let body: Value = req.body_json().await?;
    let json_str = body.to_string();
    let bo = req.state().atinyvectors_bo.clone();

    // leader generate token for cluster synchronization
    let generated_token = match bo.rbac_token.generate_jwt_token(0) {
        Ok(token) => token,
        Err(e) => return Ok(Response::builder(StatusCode::InternalServerError)
            .header("Content-Type", "application/text")
            .body(Body::from_string(e)).build()),
    };

    let wrapped_body = json!({
        "request": {
            "command": "create_rbac_token",
            "token": generated_token,
            "value": json_str
        }
    });

    let raft_req = RaftRequest::Set {
        key: "create_rbac_token".to_string(),
        value: serde_json::to_string(&wrapped_body)?,
    };

    let res = req.state().raft.client_write(raft_req).await;

    match res {
        Ok(raft_res) => Ok(Response::builder(StatusCode::Created)
            .header("Content-Type", "application/json")
            .body(Body::from_json(&json!({"result": "success", "token": generated_token}))?)
            .build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .header("Content-Type", "application/text")
            .body(Body::from_string(e.to_string())).build()),
    }
}

// GET /api/security/tokens
#[utoipa::path(
    get,
    path = "/api/security/tokens",
    responses(
        (status = 200, description = "List of RBAC tokens", body = ListRbacTokensResponse),
        (status = 500, description = "Internal Server Error", body = RbacTokenErrorResponse)
    )
)]
pub async fn list_rbac_tokens(req: Request<Arc<App>>) -> tide::Result {
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.rbac_token.list_tokens();

    match result {
        Ok(tokens) => Ok(Response::builder(StatusCode::Ok)
            .header("Content-Type", "application/json")
            .body(Body::from_string(tokens)).build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .header("Content-Type", "application/text")
            .body(Body::from_string(e)).build()),
    }
}

// DELETE /api/security/tokens/{token}
#[utoipa::path(
    delete,
    path = "/api/security/tokens/{token}",
    responses(
        (status = 200, description = "RBAC token deleted successfully", body = RbacTokenResponse),
        (status = 500, description = "Internal Server Error", body = RbacTokenErrorResponse)
    )
)]
pub async fn delete_rbac_token(req: Request<Arc<App>>) -> tide::Result {
    let token = req.param("token").unwrap_or("").to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.rbac_token.delete_token(&token);

    match result {
        Ok(_) => Ok(Response::builder(StatusCode::Ok)
            .header("Content-Type", "application/json")
            .body(json!({"status": "Token deleted successfully"})).build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .header("Content-Type", "application/text")
            .body(Body::from_string(e)).build()),
    }
}

// PUT /api/security/tokens/{token}
#[utoipa::path(
    put,
    path = "/api/security/tokens/{token}",
    request_body = RbacTokenRequest,
    responses(
        (status = 200, description = "RBAC token updated successfully", body = RbacTokenResponse),
        (status = 500, description = "Internal Server Error", body = RbacTokenErrorResponse)
    )
)]
pub async fn update_rbac_token(mut req: Request<Arc<App>>) -> tide::Result {
    let token = req.param("token").unwrap_or("").to_string();
    let body: Value = req.body_json().await?;
    let json_str = body.to_string();
    let bo = req.state().atinyvectors_bo.clone();
    let result = bo.rbac_token.update_token(&token, &json_str);

    match result {
        Ok(_) => Ok(Response::builder(StatusCode::Ok)
            .header("Content-Type", "application/json")
            .body(json!({"status": "Token updated successfully", "token": token})).build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .header("Content-Type", "application/text")
            .body(Body::from_string(e)).build()),
    }
}
