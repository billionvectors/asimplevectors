use std::fs::File;
use tracing_appender::non_blocking;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::prelude::*; 
use tracing_subscriber::filter::{EnvFilter, Directive};
use tracing::Level;
use anyhow::Result;

use dotenv::dotenv;
use config::Config;

mod atinyvectors;
mod raft_cluster;
mod service;
mod config;
mod tests;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    Config::initialize();

    let log_file = File::create(Config::service_log_file())?;
    let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);
    let file_layer = Layer::default()
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_ansi(false);
    
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_ansi(true);

    let filter_layer = EnvFilter::builder().with_env_var("ATV_SERVICE_LOG_LEVEL").try_from_env().unwrap()
        .add_directive("openraft=error".parse().unwrap())
        .add_directive("toy_rpc=error".parse().unwrap())
        .add_directive("tide=error".parse().unwrap());

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .with(filter_layer)
        .init();

    tracing::info!("Start Application");

    let id = Config::instance_id();
    let rpc_addr = Config::rpc_addr();
    let db_path = format!("{}.db", rpc_addr);
    let http_addr = Config::http_addr();

    // Start the Raft node with the retrieved configuration values
    raft_cluster::start_raft_node(
        id,
        db_path,
        http_addr,
        rpc_addr,
    )
    .await
}
