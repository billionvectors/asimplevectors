use serde_json::Value;
use std::sync::Arc;
use tracing;
use crate::atinyvectors::atinyvectors_bo::ATinyVectorsBO;

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
            "version" => self.process_version_command(request_obj).await,
            "vector" => self.process_vector_command(request_obj).await,
            "vector_with_version" => self.process_vector_with_version_command(request_obj).await,
            "create_snapshot" => self.process_create_snapshot_command(request_obj).await,
            "snapshot_restore" => self.process_snapshot_restore_command(request_obj).await,
            "create_rbac_token" => self.process_create_rbac_token_command(request_obj).await,
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

    async fn process_snapshot_restore_command(&self, request_obj: &Value) {
        let file_name = request_obj.get("file_name").and_then(|v| v.as_str()).unwrap_or("default");

        tracing::debug!("Processing snapshot_restore command: {}", file_name);

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
}
