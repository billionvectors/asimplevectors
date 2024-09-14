use std::collections::BTreeMap;
use std::sync::Arc;

use openraft::Config;
use tokio::sync::RwLock;

use crate::raft_atinyvectors::ExampleRaft;
use crate::raft_atinyvectors::NodeId;
use crate::raft_atinyvectors::ATinyVectorsRaftCommand;
use crate::raft_atinyvectors::atinyvectors_bo::ATinyVectorsBO;

// Representation of an application state. This struct can be shared around to share
// instances of raft, store and more.
pub struct App {
    pub id: NodeId,
    pub api_addr: String,
    pub rpc_addr: String,
    pub raft: ExampleRaft,
    pub key_values: Arc<RwLock<BTreeMap<String, String>>>,
    pub config: Arc<Config>,
    pub atinyvectors_bo: Arc<ATinyVectorsBO>,
    pub atinyvectors_command: Arc<ATinyVectorsRaftCommand>,
}