#![allow(clippy::uninlined_format_args)]
#![deny(unused_qualifications)]

use std::fmt::Display;
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;
use std::collections::BTreeMap;

use openraft::Config;
use tokio::net::TcpListener;
use tokio::task;

use app::App;
use network::management;
use network::Network;
use store::new_storage;
use store::Request;
use store::Response;
use async_std::process::exit;

pub mod app;
pub mod client;
pub mod network;
pub mod store;

pub type NodeId = u64;

use crate::atinyvectors::atinyvectors_raft_command::ATinyVectorsRaftCommand;
use crate::atinyvectors::atinyvectors_bo::ATinyVectorsBO;
use crate::service::routes;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct Node {
    pub rpc_addr: String,
    pub api_addr: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ rpc_addr: {}, api_addr: {} }}", self.rpc_addr, self.api_addr)
    }
}

pub type SnapshotData = Cursor<Vec<u8>>;

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
        Node = Node,
);

pub mod typ {
    use openraft::error::Infallible;

    use crate::raft_cluster::TypeConfig;

    pub type Entry = openraft::Entry<TypeConfig>;

    pub type RaftError<E = Infallible> = openraft::error::RaftError<TypeConfig, E>;
    pub type RPCError<E = Infallible> = openraft::error::RPCError<TypeConfig, RaftError<E>>;

    pub type ClientWriteError = openraft::error::ClientWriteError<TypeConfig>;
    pub type CheckIsLeaderError = openraft::error::CheckIsLeaderError<TypeConfig>;
    pub type ForwardToLeader = openraft::error::ForwardToLeader<TypeConfig>;
    pub type InitializeError = openraft::error::InitializeError<TypeConfig>;

    pub type ClientWriteResponse = openraft::raft::ClientWriteResponse<TypeConfig>;
}

pub type RaftCluster = openraft::Raft<TypeConfig>;

pub type Server = tide::Server<Arc<App>>;

pub async fn start_raft_node<P>(
    node_id: NodeId,
    dir: P,
    http_addr: String,
    rpc_addr: String,
) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    if crate::Config::raft_heartbeat_interval() > crate::Config::raft_election_timeout() {
        tracing::error!("Heatbeat interval shoud be lower than Election Timeout: election_timeout={}, heartbeat_interval={}",
            crate::Config::raft_election_timeout(), crate::Config::raft_heartbeat_interval());
        exit(-1);
    }

    if crate::Config::raft_heartbeat_interval() >= 300 {
        tracing::error!("Heatbeat interval is shoud be under 300: heartbeat_interval={}",
            crate::Config::raft_heartbeat_interval());
        exit(-1);
    }

    tracing::info!("App Server listening on: {}", http_addr);
    
    if crate::Config::enable_swagger_ui() {
        tracing::info!("Swagger running on: {}/swagger-ui/", http_addr);
    }

    // Create a configuration for the raft instance.
    let config = Config {
        heartbeat_interval: crate::Config::raft_heartbeat_interval(),
        election_timeout_min: crate::Config::raft_election_timeout(),
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    // init atinyvectors module
    let atinyvectors_bo = Arc::new(ATinyVectorsBO::new());
    let atinyvectors_command = Arc::new(ATinyVectorsRaftCommand::new(atinyvectors_bo.clone()));

    let (log_store, state_machine_store) = new_storage(&dir, atinyvectors_command.clone()).await;

    let kvs = state_machine_store.data.kvs.clone();

    // Create the network layer that will connect and communicate the raft instances and
    // will be used in conjunction with the store created above.
    let network = Network {};

    // Create a local raft instance.
    let raft = openraft::Raft::new(node_id, config.clone(), network, log_store, state_machine_store).await.unwrap();
    if crate::Config::standalone() {
        // standalone
        tracing::info!("This is standalone mode");
        let mut nodes = BTreeMap::new();
        let node = Node {
            api_addr: http_addr.clone(),
            rpc_addr: rpc_addr.clone(),
        };

        nodes.insert(node_id, node);
        raft.initialize(nodes).await;
    } else {
        tracing::info!("This is cluster mode. you should call /cluster/init before api call");
    }

    let app = Arc::new(App {
        id: node_id,
        api_addr: http_addr.clone(),
        rpc_addr: rpc_addr.clone(),
        raft,
        key_values: kvs,
        config,
        atinyvectors_bo,
        atinyvectors_command
    });

    let echo_service = Arc::new(network::raft::Raft::new(app.clone()));

    let server = toy_rpc::Server::builder().register(echo_service).build();

    let listener = TcpListener::bind(rpc_addr).await.unwrap();
    let handle = task::spawn(async move {
        server.accept_websocket(listener).await.unwrap();
    });

    let mut app: Server = tide::Server::with_state(app);

    management::rest(&mut app);
    routes::register_routes(&mut app);

    app.listen(http_addr.clone()).await?;

    _ = handle.await;
    Ok(())
}