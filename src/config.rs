use std::env;
use clap::{Arg, ArgAction, Command};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Config struct that dynamically fetches values from environment variables.
#[derive(Debug)]
pub struct Config;

// Create a singleton for Config
static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::new()));

/// Implement Config struct
impl Config {
    /// Creates a new Config instance
    pub fn new() -> Self {
        Self
    }

    /// Initialize the configuration and update environment variables if needed
    pub fn initialize() {
        let matches = Command::new("asimplevectors")
            .version(env!("CARGO_PKG_VERSION")) // Uses the version defined in Cargo.toml
            .author("billionvectors.com")
            .about("Handles config with environment variables and command-line arguments")
            .arg(
                Arg::new("cache_capacity")
                    .long("cache_capacity")
                    .action(ArgAction::Set)
                    .help("Set cache capacity for HNSW index"),
            )
            .arg(
                Arg::new("db_name")
                    .long("db_name")
                    .action(ArgAction::Set)
                    .help("Set the database name"),
            )
            .arg(
                Arg::new("log_file")
                    .long("log_file")
                    .action(ArgAction::Set)
                    .help("Set the log file path"),
            )
            .arg(
                Arg::new("log_level")
                    .long("log_level")
                    .action(ArgAction::Set)
                    .help("Set the log level (e.g., info, debug)"),
            )
            .arg(
                Arg::new("service_log_file")
                    .long("service_log_file")
                    .action(ArgAction::Set)
                    .help("Set the service log file path"),
            )
            .arg(
                Arg::new("service_log_level")
                    .long("service_log_level")
                    .action(ArgAction::Set)
                    .help("Set the service log level (e.g., info, debug)"),
            )
            .arg(
                Arg::new("m")
                    .long("m")
                    .action(ArgAction::Set)
                    .help("Set the default M value"),
            )
            .arg(
                Arg::new("ef_construction")
                    .long("ef_construction")
                    .action(ArgAction::Set)
                    .help("Set the EF construction value"),
            )
            .arg(
                Arg::new("max_datasize")
                    .long("max_datasize")
                    .action(ArgAction::Set)
                    .help("Set the maximum data size"),
            )
            .arg(
                Arg::new("data_path")
                    .long("data_path")
                    .action(ArgAction::Set)
                    .help("Set the data path"),
            )
            .arg(
                Arg::new("token_expire_days")
                    .long("token_expire_days")
                    .action(ArgAction::Set)
                    .help("Set token expiration days"),
            )
            .arg(
                Arg::new("jwt_token_key")
                    .long("jwt_token_key")
                    .action(ArgAction::Set)
                    .help("Set the JWT token key"),
            )
            .arg(
                Arg::new("enable_security")
                    .long("enable_security")
                    .action(ArgAction::Set)
                    .help("Enable or disable security (1: enable or 0: disable)"),
            )
            .arg(
                Arg::new("enable_swagger_ui")
                    .long("enable_swagger_ui")
                    .action(ArgAction::SetTrue)
                    .help("Enable or disable Swagger UI (true: enable, false: disable)"),
            )
            .arg(
                Arg::new("id")
                    .long("id")
                    .action(ArgAction::Set)
                    .help("Set the instance ID"),
            )
            .arg(
                Arg::new("standalone")
                    .long("standalone")
                    .action(ArgAction::SetTrue)
                    .help("Set the standalone node (true: standalone, false: cluster)"),
            )
            .arg(
                Arg::new("http_addr")
                    .long("http-addr")
                    .action(ArgAction::Set)
                    .help("Set the HTTP address"),
            )
            .arg(
                Arg::new("rpc_addr")
                    .long("rpc-addr")
                    .action(ArgAction::Set)
                    .help("Set the RPC address"),
            )
            .arg(
                Arg::new("raft_heartbeat_interval")
                    .long("raft_heartbeat_interval")
                    .action(ArgAction::Set)
                    .help("Set the Raft heartbeat interval (ms)"),
            )
            .arg(
                Arg::new("raft_election_timeout")
                    .long("raft_election_timeout")
                    .action(ArgAction::Set)
                    .help("Set the Raft election timeout (ms)"),
            )
            .get_matches();

        // Check and update environment variables from command-line arguments
        if let Some(value) = matches.get_one::<String>("cache_capacity") {
            env::set_var("ATV_HNSW_INDEX_CACHE_CAPACITY", value);
        }

        if let Some(value) = matches.get_one::<String>("db_name") {
            env::set_var("ATV_DB_NAME", value);
        }

        if let Some(value) = matches.get_one::<String>("log_file") {
            env::set_var("ATV_LOG_FILE", value);
        }

        if let Some(value) = matches.get_one::<String>("log_level") {
            env::set_var("ATV_LOG_LEVEL", value);
        }

        if let Some(value) = matches.get_one::<String>("service_log_file") {
            env::set_var("ATV_SERVICE_LOG_FILE", value);
        }

        if let Some(value) = matches.get_one::<String>("service_log_level") {
            env::set_var("ATV_SERVICE_LOG_LEVEL", value);
        }

        if let Some(value) = matches.get_one::<String>("m") {
            env::set_var("ATV_DEFAULT_M", value);
        }

        if let Some(value) = matches.get_one::<String>("ef_construction") {
            env::set_var("ATV_DEFAULT_EF_CONSTRUCTION", value);
        }

        if let Some(value) = matches.get_one::<String>("max_datasize") {
            env::set_var("ATV_HNSW_MAX_DATASIZE", value);
        }

        if let Some(value) = matches.get_one::<String>("data_path") {
            env::set_var("ATV_DATA_PATH", value);
        }

        if let Some(value) = matches.get_one::<String>("token_expire_days") {
            env::set_var("ATV_DEFAULT_TOKEN_EXPIRE_DAYS", value);
        }

        if let Some(value) = matches.get_one::<String>("jwt_token_key") {
            env::set_var("ATV_JWT_TOKEN_KEY", value);
        }

        if let Some(value) = matches.get_one::<String>("enable_security") {
            env::set_var("ATV_ENABLE_SECURITY", value);
        }

        if matches.get_flag("enable_swagger_ui") {
            env::set_var("ATV_ENABLE_SWAGGER_UI", "true");
        }

        if let Some(value) = matches.get_one::<String>("id") {
            env::set_var("ATV_INSTANCE_ID", value);
        }

        if matches.get_flag("standalone") {
            env::set_var("ATV_STANDALONE", "true");
        }

        if let Some(value) = matches.get_one::<String>("http_addr") {
            env::set_var("ATV_HTTP_ADDR", value);
        }

        if let Some(value) = matches.get_one::<String>("rpc_addr") {
            env::set_var("ATV_RPC_ADDR", value);
        }

        if let Some(value) = matches.get_one::<String>("raft_heartbeat_interval") {
            env::set_var("ATV_RAFT_HEARTBEAT_INTERVAL", value);
        }

        if let Some(value) = matches.get_one::<String>("raft_election_timeout") {
            env::set_var("ATV_RAFT_ELECTION_TIMEOUT", value);
        }
    }

    // Dynamic getters that always read from the environment
    pub fn cache_capacity() -> i64 {
        env::var("ATV_HNSW_INDEX_CACHE_CAPACITY")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<i64>()
            .unwrap_or(100)
    }

    pub fn db_name() -> String {
        env::var("ATV_DB_NAME").unwrap_or_else(|_| ":memory:".to_string())
    }

    pub fn log_file() -> String {
        env::var("ATV_LOG_FILE").unwrap_or_else(|_| "logs/atinyvectors.log".to_string())
    }

    pub fn log_level() -> String {
        env::var("ATV_LOG_LEVEL").unwrap_or_else(|_| "info".to_string())
    }

    pub fn service_log_file() -> String {
        env::var("ATV_SERVICE_LOG_FILE").unwrap_or_else(|_| "logs/asimplevectors.log".to_string())
    }

    pub fn service_log_level() -> String {
        env::var("ATV_SERVICE_LOG_LEVEL").unwrap_or_else(|_| "info".to_string())
    }

    pub fn default_m() -> i64 {
        env::var("ATV_DEFAULT_M")
            .unwrap_or_else(|_| "16".to_string())
            .parse::<i64>()
            .unwrap_or(16)
    }

    pub fn ef_construction() -> i64 {
        env::var("ATV_DEFAULT_EF_CONSTRUCTION")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<i64>()
            .unwrap_or(100)
    }

    pub fn max_datasize() -> i64 {
        env::var("ATV_HNSW_MAX_DATASIZE")
            .unwrap_or_else(|_| "1000000".to_string())
            .parse::<i64>()
            .unwrap_or(1_000_000)
    }

    pub fn data_path() -> String {
        env::var("ATV_DATA_PATH").unwrap_or_else(|_| "data/".to_string())
    }

    pub fn token_expire_days() -> i64 {
        env::var("ATV_DEFAULT_TOKEN_EXPIRE_DAYS")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<i64>()
            .unwrap_or(30)
    }

    pub fn jwt_token_key() -> String {
        env::var("ATV_JWT_TOKEN_KEY").unwrap_or_else(|_| {
            "atinyvectors_jwt_token_key_is_really_good_and_i_hope_so_much_whatever_you_want".to_string()
        })
    }

    pub fn enable_security() -> i64 {
        env::var("ATV_ENABLE_SECURITY")
            .unwrap_or_else(|_| "0".to_string())
            .parse::<i64>()
            .unwrap_or(0)
    }

    pub fn enable_swagger_ui() -> bool {
        env::var("ATV_ENABLE_SWAGGER_UI")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false)
    }

    pub fn standalone() -> bool {
        env::var("ATV_STANDALONE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false)
    }

    pub fn instance_id() -> u64 {
        env::var("ATV_INSTANCE_ID")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<u64>()
        .unwrap_or(1)
    }

    pub fn http_addr() -> String {
        env::var("ATV_HTTP_ADDR").unwrap_or_else(|_| "0.0.0.0:21001".to_string())
    }

    pub fn rpc_addr() -> String {
        env::var("ATV_RPC_ADDR").unwrap_or_else(|_| "0.0.0.0:22001".to_string())
    }

    pub fn raft_heartbeat_interval() -> u64 {
        env::var("ATV_RAFT_HEARTBEAT_INTERVAL")
            .unwrap_or_else(|_| "250".to_string())
            .parse::<u64>()
            .unwrap_or(250)
    }

    pub fn raft_election_timeout() -> u64 {
        env::var("ATV_RAFT_ELECTION_TIMEOUT")
            .unwrap_or_else(|_| "299".to_string())
            .parse::<u64>()
            .unwrap_or(299)
    }

    /// Method to get the singleton Config instance
    pub fn get_config() -> &'static Mutex<Config> {
        &CONFIG
    }
}
