use std::sync::Arc;
use tide::Server;

use crate::raft_cluster::app::App;
use crate::service::handlers::
    {search_handler, security_handler, snapshot_handler, space_handler, vector_handler, version_handler};

pub fn register_routes(app: &mut Server<Arc<App>>) {
    let mut api = app.at("/api");

    // Space endpoints
    api.at("/space/:space_name").get(space_handler::get_space);
    api.at("/space").post(space_handler::space);
    api.at("/space/list").get(space_handler::list_spaces);

    // Version endpoints
    api.at("/space/:space_name/version/list").get(version_handler::list_versions);
    api.at("/space/:space_name/version/:version_id").get(version_handler::get_version_by_id);
    api.at("/space/:space_name/version/:version_name/by-name").get(version_handler::get_version_by_name);
    api.at("/space/:space_name/version/default").get(version_handler::get_default_version);
    api.at("/space/:space_name/version").post(version_handler::create_version);

    // Vector endpoints
    api.at("/space/:space_name/vector").post(vector_handler::vector);
    api.at("/space/:space_name/version/:version_id/vector").post(vector_handler::vector_with_version);
    api.at("/space/:space_name/version/:version_id/vectors").get(vector_handler::get_vectors_by_version_id);
    
    // Search endpoints
    api.at("/space/:space_name/search").post(search_handler::search);
    api.at("/space/:space_name/version/:version_id/search").post(search_handler::search_with_version);

    // Snapshot endpoints
    api.at("/snapshot").post(snapshot_handler::create_snapshot);
    api.at("/snapshot/:file_name/download").get(snapshot_handler::download_snapshot);
    api.at("/snapshot/:file_name/restore").post(snapshot_handler::restore_snapshot);
    api.at("/snapshots").get(snapshot_handler::list_snapshots);
    api.at("/snapshots/delete").delete(snapshot_handler::delete_snapshots);
    api.at("/snapshots/restore").post(snapshot_handler::restore_snapshot_from_upload);

    // Security endpoints
    api.at("/security/tokens").post(security_handler::create_rbac_token);
    api.at("/security/tokens").get(security_handler::list_rbac_tokens);
    api.at("/security/tokens/:token").delete(security_handler::delete_rbac_token);
    api.at("/security/tokens/:token").put(security_handler::update_rbac_token);
}
