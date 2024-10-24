use std::env;
use crate::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            Config::initialize();
        });
    }

    #[test]
    fn test_override_env_values() {
        // Set environment variables
        env::set_var("ATV_HNSW_INDEX_CACHE_CAPACITY", "200");
        env::set_var("ATV_DB_NAME", "test_db");
        env::set_var("ATV_LOG_FILE", "test.log");
        env::set_var("ATV_LOG_LEVEL", "debug");
        env::set_var("ATV_DEFAULT_M", "32");
        env::set_var("ATV_DEFAULT_EF_CONSTRUCTION", "200");
        env::set_var("ATV_HNSW_MAX_DATASIZE", "2000000");
        env::set_var("ATV_DATA_PATH", "test_data/");
        env::set_var("ATV_DEFAULT_TOKEN_EXPIRE_DAYS", "60");
        env::set_var("ATV_JWT_TOKEN_KEY", "test_key");
        env::set_var("ATV_ENABLE_SECURITY", "1");

        Config::initialize(); // Re-initialize to pick up new env vars

        // Ensure updated values are loaded correctly
        assert_eq!(Config::cache_capacity(), 200);
        assert_eq!(Config::db_name(), "test_db");
        assert_eq!(Config::log_file(), "test.log");
        assert_eq!(Config::log_level(), "debug");
        assert_eq!(Config::default_m(), 32);
        assert_eq!(Config::ef_construction(), 200);
        assert_eq!(Config::max_datasize(), 2_000_000);
        assert_eq!(Config::data_path(), "test_data/");
        assert_eq!(Config::token_expire_days(), 60);
        assert_eq!(Config::jwt_token_key(), "test_key");
        assert_eq!(Config::enable_security(), 1);
    }
}
