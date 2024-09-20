use tracing_subscriber::EnvFilter;
mod atinyvectors;
mod raft_cluster;
mod service;
mod config;

use dotenv::dotenv;
use config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    Config::initialize();

    // Setup the logger with configurations from environment variables
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_ansi(false)
        .with_env_filter(EnvFilter::new(Config::log_level()))
        .init();

    let id = Config::instance_id();
    let rpc_addr = Config::rpc_addr();
    let db_path = format!("{}.db", rpc_addr);
    let http_addr = Config::http_addr();

    // Start the Raft node with the retrieved configuration values
    raft_cluster::start_example_raft_node(
        id,
        db_path,
        http_addr,
        rpc_addr,
    )
    .await
}
