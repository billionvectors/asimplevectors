use std::sync::Arc;
use tide::Server;
use tide::prelude::json;
use tide::{http::Mime, Response};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::Config;

use crate::raft_cluster::app::App;
use crate::service::handlers::{
    kvstorage_handler,
    rerank_handler, search_handler, security_handler, 
    snapshot_handler, space_handler, vector_handler, 
    version_handler,
};

use crate::service::handlers::dto::keyvalue_dto::{
    KeyValueRequest, KeyValueResponse, KeyValueErrorResponse, ListKeysResponse};

use crate::service::handlers::dto::rerank_dto::{
    RerankRequest, RerankResponse, RerankErrorResponse};
    
use crate::service::handlers::dto::search_dto::{
    SearchRequest, SearchResponse, SearchErrorResponse};
    
use crate::service::handlers::dto::security_dto::{
    RbacTokenRequest, RbacTokenResponse, RbacTokenErrorResponse, ListRbacTokensResponse, TokenDetails};

use crate::service::handlers::dto::snapshot_dto::{
    CreateSnapshotRequest, SnapshotResponse, SnapshotErrorResponse, ListSnapshotsResponse, SnapshotInfo};
    
use crate::service::handlers::dto::space_dto::{
    SpaceRequest, SpaceResponse, SpaceErrorResponse, 
    DenseConfig, HnswConfig, QuantizationConfig, 
    SparseConfig, ScalarQuantizationConfig, ProductQuantizationConfig,
    VersionData, VectorIndexData, ListSpacesResponse, SpaceInfo};

use crate::service::handlers::dto::vector_dto::{
    VectorData, VectorRequest, VectorResponse, VectorErrorResponse, GetVectorsResponse, VectorDataResponse};
    
use crate::service::handlers::dto::version_dto::{
    VersionRequest, VersionResponse, VersionErrorResponse, ListVersionsResponse, VersionInfo};
    
async fn serve_swagger(request: tide::Request<Arc<App>>) -> tide::Result<Response> {
    // swagger config
    let swagger_config = Arc::new(utoipa_swagger_ui::Config::from("/api-docs/openapi.json"));

    let mut path = request.url().path().to_string();

    // Ensure the path ends with a trailing slash if it's /swagger-ui
    if path == "/swagger-ui" {
        path.push_str("/");
    }

    let tail = path.strip_prefix("/swagger-ui/").unwrap_or_default();

    match utoipa_swagger_ui::serve(tail, swagger_config) {
        Ok(swagger_file) => swagger_file
            .map(|file| {
                Ok(Response::builder(200)
                    .body(file.bytes.to_vec())
                    .content_type(file.content_type.parse::<Mime>()?)
                    .build())
            })
            .unwrap_or_else(|| Ok(Response::builder(404).build())),
        Err(error) => Ok(Response::builder(500).body(error.to_string()).build()),
    }
}
    

pub fn build_openapi(app: &mut Server<Arc<App>>) {
    if !crate::Config::enable_swagger_ui() {
        return;
    }

    #[derive(OpenApi)]
    #[openapi(
        paths(
            kvstorage_handler::put_key,
            kvstorage_handler::get_key,
            kvstorage_handler::remove_key,
            kvstorage_handler::list_keys,

            rerank_handler::rerank,
            rerank_handler::rerank_with_version,

            search_handler::search,
            search_handler::search_with_version,

            security_handler::create_rbac_token,
            security_handler::list_rbac_tokens,
            security_handler::delete_rbac_token,
            security_handler::update_rbac_token,

            snapshot_handler::create_snapshot,
            snapshot_handler::restore_snapshot,
            snapshot_handler::delete_snapshot,
            snapshot_handler::list_snapshots,
            snapshot_handler::delete_all_snapshots,
            snapshot_handler::download_snapshot,

            space_handler::space,
            space_handler::get_space,
            space_handler::delete_space,
            space_handler::list_spaces,

            vector_handler::vector,
            vector_handler::vector_with_version,
            vector_handler::get_vectors_by_version_id,

            version_handler::create_version,
            version_handler::get_version_by_id,
            version_handler::get_version_by_name,
            version_handler::get_default_version,
            version_handler::list_versions,
        ),
        tags(
            (name = "space", description = "space items management endpoints.")
        ),
        components(
            schemas(
                KeyValueRequest, KeyValueResponse, KeyValueErrorResponse, ListKeysResponse,

                RerankRequest, RerankResponse, RerankErrorResponse,
                SearchRequest, SearchResponse, SearchErrorResponse,
                
                RbacTokenRequest, RbacTokenResponse, RbacTokenErrorResponse, ListRbacTokensResponse, TokenDetails,

                CreateSnapshotRequest, SnapshotResponse, SnapshotErrorResponse, ListSnapshotsResponse, SnapshotInfo,

                SpaceRequest, SpaceResponse, SpaceErrorResponse, 
                DenseConfig, HnswConfig, QuantizationConfig, 
                SparseConfig, ScalarQuantizationConfig, ProductQuantizationConfig,
                VersionData, VectorIndexData, ListSpacesResponse, SpaceInfo,

                VectorData, VectorRequest, VectorResponse, VectorErrorResponse, GetVectorsResponse, VectorDataResponse,

                VersionRequest, VersionResponse, VersionErrorResponse, ListVersionsResponse, VersionInfo,
            )
        )
    )]
    struct ApiDoc;

    // serve OpenApi json
    app.at("/api-docs/openapi.json")
        .get(|_| async move { Ok(Response::builder(200).body(json!(ApiDoc::openapi()))) });
    
    // serve Swagger UI
    app.at("/swagger-ui/*").get(serve_swagger);
}

pub fn register_routes(app: &mut Server<Arc<App>>) {
    // Create an application that will store all the instances created above, this will
    // be later used on the actix-web services.

    build_openapi(app);

    // end points
    let mut api = app.at("/api");

    // keyvalue Storage endpoints
    api.at("/space/:space_name/key/:key").post(kvstorage_handler::put_key);
    api.at("/space/:space_name/key/:key").get(kvstorage_handler::get_key);
    api.at("/space/:space_name/key/:key").delete(kvstorage_handler::remove_key);
    api.at("/space/:space_name/keys").get(kvstorage_handler::list_keys);

    // Rerank endpoints
    api.at("/space/:space_name/rerank").post(rerank_handler::rerank);
    api.at("/space/:space_name/version/:version_id/rerank").post(rerank_handler::rerank_with_version);
    
    // Search endpoints (default index name is "default")
    api.at("/space/:space_name/search").post(search_handler::search);
    api.at("/space/:space_name/search/:index_name").post(search_handler::search);
    api.at("/space/:space_name/version/:version_id/search").post(search_handler::search_with_version);
    api.at("/space/:space_name/version/:version_id/search/:index_name").post(search_handler::search_with_version);

    // Security endpoints
    api.at("/security/tokens").post(security_handler::create_rbac_token);
    api.at("/security/tokens").get(security_handler::list_rbac_tokens);
    api.at("/security/tokens/:token").delete(security_handler::delete_rbac_token);
    api.at("/security/tokens/:token").put(security_handler::update_rbac_token);

    // Snapshot endpoints
    api.at("/snapshot").post(snapshot_handler::create_snapshot);
    api.at("/snapshot/:file_name/download").get(snapshot_handler::download_snapshot);
    api.at("/snapshot/:file_name/restore").post(snapshot_handler::restore_snapshot);
    api.at("/snapshot/:file_name/delete").delete(snapshot_handler::delete_snapshot);
    api.at("/snapshots").get(snapshot_handler::list_snapshots);
    api.at("/snapshots/restore").post(snapshot_handler::restore_snapshot_from_upload);
    api.at("/snapshot/delete_all").delete(snapshot_handler::delete_all_snapshots);

    // Space endpoints
    api.at("/space").post(space_handler::space);
    api.at("/space/:space_name").get(space_handler::get_space);
    api.at("/space/:space_name").post(space_handler::update_space);
    api.at("/space/:space_name").delete(space_handler::delete_space);
    api.at("/spaces").get(space_handler::list_spaces);

    // Vector endpoints (default index name is "default")
    api.at("/space/:space_name/vector").post(vector_handler::vector);
    api.at("/space/:space_name/vector/:index_name").post(vector_handler::vector);
    api.at("/space/:space_name/version/:version_id/vector").post(vector_handler::vector_with_version);
    api.at("/space/:space_name/version/:version_id/vector/:index_name").post(vector_handler::vector_with_version);
    api.at("/space/:space_name/version/:version_id/vectors").get(vector_handler::get_vectors_by_version_id);
    api.at("/space/:space_name/version/:version_id/vectors/:index_name").get(vector_handler::get_vectors_by_version_id);
    api.at("/space/:space_name/vectors").get(vector_handler::get_vectors_by_default_version);

    // Version endpoints
    api.at("/space/:space_name/versions").get(version_handler::list_versions);
    api.at("/space/:space_name/version/:version_id").get(version_handler::get_version_by_id);
    api.at("/space/:space_name/version/:version_name/by-name").get(version_handler::get_version_by_name);
    api.at("/space/:space_name/version").get(version_handler::get_default_version);
    api.at("/space/:space_name/version").post(version_handler::create_version);
    api.at("/space/:space_name/version/:version_id").delete(version_handler::delete_version);
}
