use std::sync::Arc;

use openraft::raft::AppendEntriesRequest;
use openraft::raft::AppendEntriesResponse;
use openraft::raft::InstallSnapshotRequest;
use openraft::raft::InstallSnapshotResponse;
use openraft::raft::VoteRequest;
use openraft::raft::VoteResponse;
use toy_rpc::macros::export_impl;

use crate::raft_cluster::app::App;
use crate::raft_cluster::TypeConfig;

/// Raft protocol service.
pub struct Raft {
    app: Arc<App>,
}

#[export_impl]
impl Raft {
    pub fn new(app: Arc<App>) -> Self {
        Self { app }
    }

    #[export_method]
    pub async fn vote(&self, vote: VoteRequest<TypeConfig>) -> Result<VoteResponse<TypeConfig>, toy_rpc::Error> {
        self.app.raft.vote(vote).await.map_err(|e| toy_rpc::Error::Internal(Box::new(e)))
    }

    #[export_method]
    pub async fn append(
        &self,
        req: AppendEntriesRequest<TypeConfig>,
    ) -> Result<AppendEntriesResponse<TypeConfig>, toy_rpc::Error> {
        self.app.raft.append_entries(req).await.map_err(|e| toy_rpc::Error::Internal(Box::new(e)))
    }

    #[export_method]
    pub async fn snapshot(
        &self,
        req: InstallSnapshotRequest<TypeConfig>,
    ) -> Result<InstallSnapshotResponse<TypeConfig>, toy_rpc::Error> {
        self.app.raft.install_snapshot(req).await.map_err(|e| toy_rpc::Error::Internal(Box::new(e)))
    }
}