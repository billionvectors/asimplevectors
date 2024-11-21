use serde_json::Value;
use std::sync::Arc;
use tracing;
use rocksdb::{Options, DB};
use async_std::fs;
use async_std::fs::File;
use async_std::path::Path;
use async_std::path::PathBuf;
use async_std::io::WriteExt;
use reqwest;
use reqwest::StatusCode;
use regex::Regex;

use crate::{atinyvectors::atinyvectors_bo::ATinyVectorsBO, config::Config};

#[derive(Clone, Debug)]
pub struct ATinyVectorsRaftCommand {
    pub atinyvectors_bo: Arc<ATinyVectorsBO>,
}

impl ATinyVectorsRaftCommand {
    pub fn new(atinyvectors_bo: Arc<ATinyVectorsBO>) -> Self {
        Self { atinyvectors_bo: atinyvectors_bo.clone() }
    }

    pub async fn process_command(
        &self,
        command: &str,
        request_obj: &Value,
        key: &str,
        value: &str,
    ) {
        match command {
            "space" => self.process_space_command(request_obj, key, value).await,
            "update_space" => self.process_update_space_command(request_obj).await,
            "delete_space" => self.process_delete_space_command(request_obj).await,
            "version" => self.process_version_command(request_obj).await,
            "vector" => self.process_vector_command(request_obj).await,
            "vector_with_version" => self.process_vector_with_version_command(request_obj).await,
            "create_snapshot" => self.process_create_snapshot_command(request_obj).await,
            "snapshot_restore" => self.process_snapshot_restore_command(request_obj).await,
            "snapshot_delete" => self.process_snapshot_delete_command(request_obj).await,
            "snapshot_sync" => self.process_snapshot_sync_command(request_obj).await,
            "create_rbac_token" => self.process_create_rbac_token_command(request_obj).await,
            "storage_put_key" => self.process_storage_put_key_command(request_obj).await,
            "storage_remove_key" => self.process_storage_remove_key_command(request_obj).await,
            _ => {
                tracing::warn!("Unknown command: {}", command);
            }
        }
    }

    async fn process_space_command(
        &self,
        request_obj: &Value,
        key: &str,
        value: &str,
    ) {
        tracing::info!("Processing space command");
        if let Some(space_value) = request_obj.get("value") {
            if let Err(e) = self.atinyvectors_bo.space.create_space(&space_value.to_string()) {
                tracing::error!("Failed to create space: {}", e);
            }
        } else {
            tracing::error!("No 'value' field found in 'request'");
        }
    }

    async fn process_update_space_command(&self, request_obj: &Value) {
        tracing::info!("Processing update space command");
        let space_name = request_obj.get("space_name").and_then(|v| v.as_str()).unwrap_or("default");
        if let Some(space_value) = request_obj.get("value") {
            if let Err(e) = self.atinyvectors_bo.space.update_space(space_name, &space_value.to_string()) {
                tracing::error!("Failed to update space: {}", e);
            }
        } else {
            tracing::error!("No 'value' field found in 'request'");
        }
    }

    async fn process_delete_space_command(&self, request_obj: &Value) {
        tracing::info!("Processing delete space command");
        let space_name = request_obj.get("space_name").and_then(|v| v.as_str()).unwrap_or("default");
        let value = request_obj.get("value").and_then(|v| v.as_str()).unwrap_or("{}");

        if let Err(e) = self.atinyvectors_bo.space.delete_space(space_name, &value.to_string()) {
            tracing::error!("Failed to delete space: {}", e);
        }
    }

    async fn process_version_command(&self, request_obj: &Value) {
        tracing::info!("Processing version command");
        let space_name = request_obj.get("space_name").and_then(|v| v.as_str()).unwrap_or("default");

        if let Some(version_value) = request_obj.get("value") {
            if let Err(e) = self.atinyvectors_bo.version.create_version(space_name, &version_value.to_string()) {
                tracing::error!("Failed to version vector: {}", e);
            }
        } else {
            tracing::error!("No 'value' field found in 'request'");
        }
    }

    async fn process_vector_command(&self, request_obj: &Value) {
        tracing::debug!("Processing vector command");
        let space_name = request_obj.get("space_name").and_then(|v| v.as_str()).unwrap_or("default");
        let version_id = 0; // default

        if let Some(version_value) = request_obj.get("value") {
            if let Err(e) = self.atinyvectors_bo.vector.upsert_vectors(space_name, version_id, &version_value.to_string()) {
                tracing::error!("Failed to upsert vector: {}", e);
            }
        } else {
            tracing::error!("No 'value' field found in 'request'");
        }
    }

    async fn process_vector_with_version_command(&self, request_obj: &Value) {
        tracing::debug!("Processing vector_with_version command");
        let space_name = request_obj.get("space_name").and_then(|v| v.as_str()).unwrap_or("default");
        let version_id = request_obj.get("version_id").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

        if let Some(vector_value) = request_obj.get("value") {
            if let Err(e) = self.atinyvectors_bo.vector.upsert_vectors(space_name, version_id, &vector_value.to_string()) {
                tracing::error!("Failed to upsert vector with version: {}", e);
            }
        } else {
            tracing::error!("No 'value' field found in 'request'");
        }
    }

    async fn process_create_snapshot_command(&self, request_obj: &Value) {
        if let Some(snapshot_value) = request_obj.get("value") {
            tracing::debug!("Processing process_create_snapshot_command command: {}", snapshot_value);
            if let Err(e) = self.atinyvectors_bo.snapshot.create_snapshot(&snapshot_value.to_string()) {
                tracing::error!("Failed to create snapshot: {}", e);
            }
        } else {
            tracing::error!("No 'value' field found in 'request'");
        }
    }

    async fn process_snapshot_delete_command(&self, request_obj: &Value) {
        let file_name = request_obj.get("file_name").and_then(|v| v.as_str()).unwrap_or("default");

        tracing::debug!("Processing process_snapshot_delete_command command: {}", file_name);

        if let Err(e) = self.atinyvectors_bo.snapshot.delete_snapshot(file_name) {
            tracing::error!("Failed to delete snapshot: {}", e);
        }
    }

    async fn process_snapshot_restore_command(&self, request_obj: &Value) {
        let file_name = request_obj.get("file_name").and_then(|v| v.as_str()).unwrap_or("default");

        tracing::debug!("Processing snapshot_restore command: {}", file_name);

        if let Err(e) = self.atinyvectors_bo.snapshot.restore_snapshot(file_name) {
            tracing::error!("Failed to restore snapshot: {}", e);
        }
    }

    async fn process_snapshot_sync_command(&self, request_obj: &Value) {
        let current_addr = Config::http_addr().clone();
        let file_name = request_obj.get("file_name").and_then(|v| v.as_str()).unwrap_or("default");
        let leader_id = request_obj.get("leader_id").and_then(|v| v.as_u64()).unwrap_or(Config::instance_id());
        let leader_addr =  request_obj.get("leader_addr").and_then(|v| v.as_str()).unwrap_or(current_addr.as_str());

        tracing::info!("Processing snapshot_sync command: file_name={} / leader_addr={}", file_name, leader_addr);

        if leader_id != Config::instance_id() {
            tracing::debug!("Not Leader Node: Trying to download snapshot file");
    
            let snapshot_dir = PathBuf::from(Config::data_path()).join("snapshot");
            tracing::debug!("Snapshot directory path: {:?}", snapshot_dir);

            if !snapshot_dir.exists().await {
                tracing::debug!("Snapshot directory does not exist. Creating...");
                fs::create_dir_all(&snapshot_dir).await.map_err(|e| {
                    tracing::error!("Failed to create snapshot directory: {}", e);
                });
                tracing::debug!("Snapshot directory created successfully");
            }
            
            // download from leader_addr
            let date = self.extract_date_from_file_name(file_name).unwrap_or_else(|| "unknown_date".to_string());
            let download_url = format!("{}/snapshot/{}/download", leader_addr, date);
            tracing::debug!("Download Endpoint: {}", download_url);

            let file_path = snapshot_dir.join(file_name);
            match self.download_file(&download_url, &file_path).await {
                Ok(_) => tracing::debug!("File downloaded successfully: {:?}", file_path),
                Err(e) => tracing::error!("Failed to download file: {}", e),
            }
        }

        if let Err(e) = self.atinyvectors_bo.snapshot.restore_snapshot(file_name) {
            tracing::error!("Failed to restore snapshot: {}", e);
        }
    }

    async fn process_create_rbac_token_command(&self, request_obj: &Value) {
        tracing::debug!("Processing create_rbac_token command");

        let token = request_obj.get("token").and_then(|v| v.as_str()).unwrap_or("");
        let json_str = request_obj.get("value").and_then(|v| v.as_str()).unwrap_or("");

        if let Err(e) = self.atinyvectors_bo.rbac_token.new_token(json_str, token) {
            tracing::error!("Failed to create RBAC token: {}", e);
        }
    }

    async fn process_storage_put_key_command(&self, request_obj: &Value) {
        tracing::debug!("Processing storage_put_key command");
        let space_name = request_obj.get("space_name").and_then(|v| v.as_str()).unwrap_or("default");
        let key = request_obj.get("key").and_then(|v| v.as_str()).unwrap_or("");
    
        if let Some(value) = request_obj.get("value") {
            if let Some(value_str) = value.as_str() {
                let target_directory = format!("{}/space/{}", Config::data_path(), space_name);
            
                // Create target directory if it does not exist
                if !std::path::Path::new(&target_directory).exists() {
                    let _ = fs::create_dir_all(&target_directory).await;
                }
    
                let path = target_directory + "storage.rocksdb";
                let mut db_opts = Options::default();
                db_opts.create_if_missing(true);
    
                let db = DB::open(&db_opts, path).unwrap();
                let _ = db.put(key, value_str);
            } else {
                tracing::error!("'value' is not a string.");
            }
        } else {
            tracing::error!("No 'value' field found in 'request'");
        }
    }    

    async fn process_storage_remove_key_command(&self, request_obj: &Value) {
        tracing::debug!("Processing storage_remove_key command");
        let space_name = request_obj.get("space_name").and_then(|v| v.as_str()).unwrap_or("default");
        let key = request_obj.get("key").and_then(|v| v.as_str()).unwrap_or("");

        let target_directory = format!("{}/space/{}", Config::data_path(), space_name);

        // Create target directory if it does not exist
        if !std::path::Path::new(&target_directory).exists() {
            let _ = fs::create_dir_all(&target_directory).await;
        }

        let path = target_directory + "storage.rocksdb";
        if !std::path::Path::new(&path).exists() {
            tracing::debug!("No database found at path: {}. Returning early.", path);
            return; // No database, no key to remove
        }

        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);

        let db = DB::open(&db_opts, path).unwrap();
        let _ = db.delete(key);
    }

    fn extract_date_from_file_name(&self, file_name: &str) -> Option<String> {
        let re = Regex::new(r"snapshot-(\d{8})\.zip").ok()?;
        re.captures(file_name).and_then(|cap| cap.get(1).map(|date| date.as_str().to_string()))
    }

    async fn download_file(&self, url: &str, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let mut file = File::create(file_path).await?;
        let content = response.bytes().await?;
    
        file.write_all(&content).await?;
        Ok(())
    }
}
