use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use regex::Regex;
use tracing::{debug, error};
use async_std::path::Path;
use async_std::path::PathBuf;
use async_std::fs;
use async_std::stream::StreamExt;

use crate::Config;

// FFI declaration for SnapshotServiceManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SnapshotServiceManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_snapshot_service_manager_new() -> *mut SnapshotServiceManager;
    pub fn atv_snapshot_service_manager_free(manager: *mut SnapshotServiceManager);
    pub fn atv_snapshot_service_create_snapshot(manager: *mut SnapshotServiceManager, json_str: *const c_char);
    pub fn atv_snapshot_service_restore_snapshot(manager: *mut SnapshotServiceManager, file_name: *const c_char);
    pub fn atv_snapshot_service_delete_snapshot(manager: *mut SnapshotServiceManager, file_name: *const c_char);
    pub fn atv_snapshot_service_list_snapshots(manager: *mut SnapshotServiceManager) -> *mut c_char;
    pub fn atv_snapshot_service_delete_snapshots(manager: *mut SnapshotServiceManager);
}

// Safe Rust wrapper for SnapshotServiceManager
#[derive(Clone, Debug)]
pub struct SnapshotServiceManagerWrapper {
    inner: *mut SnapshotServiceManager,
}

impl SnapshotServiceManagerWrapper {
    pub fn new() -> Self {
        unsafe { SnapshotServiceManagerWrapper { inner: atv_snapshot_service_manager_new() } }
    }

    fn to_ascii_string(input: &str) -> Result<String, std::string::FromUtf8Error> {
        let ascii_bytes: Vec<u8> = input
            .chars()
            .filter_map(|c| {
                if c.is_ascii() {
                    Some(c as u8)
                } else {
                    None
                }
            })
            .collect();
    
        String::from_utf8(ascii_bytes)
    }

    pub fn create_snapshot(&self, json_str: &str) -> Result<(), String> {
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_snapshot_service_create_snapshot(self.inner, json_str_c.as_ptr());
        };

        Ok(())
    }

    pub fn restore_snapshot(&self, file_name_str: &str) -> Result<(), String> {
        let file_name_str_c = CString::new(
            Self::to_ascii_string(file_name_str).unwrap().as_str()).unwrap();
        unsafe {
            atv_snapshot_service_restore_snapshot(self.inner, file_name_str_c.as_ptr());
        };

        Ok(())
    }

    pub fn delete_snapshot(&self, file_name_str: &str) -> Result<(), String> {
        let file_name_str_c = CString::new(
            Self::to_ascii_string(file_name_str).unwrap().as_str()).unwrap();
        unsafe {
            atv_snapshot_service_delete_snapshot(self.inner, file_name_str_c.as_ptr());
        };

        Ok(())
    }

    pub fn list_snapshots(&self) -> Result<String, String> {
        unsafe {
            let result = atv_snapshot_service_list_snapshots(self.inner);
            if result.is_null() {
                Err("Failed to list snapshots".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn delete_snapshots(&self)-> Result<(), String> {
        unsafe {
            atv_snapshot_service_delete_snapshots(self.inner);
        };

        Ok(())
    }

    pub async fn download_snapshot(
        &self,
        file_name: &str,
    ) -> Result<PathBuf, String> {
        let file_name = Self::to_ascii_string(file_name).unwrap();
        debug!("Starting download_snapshot function: file_name={}", file_name);

        let data_path = Config::data_path().clone() + "/snapshot/";
        let snapshot_dir = Path::new(&data_path);

        if !snapshot_dir.exists().await {
            error!("Snapshot directory does not exist: {:?}", snapshot_dir);
            return Err(format!("Snapshot directory does not exist: {:?}", snapshot_dir));
        }
        if !snapshot_dir.is_dir().await {
            error!("Snapshot path is not a directory: {:?}", snapshot_dir);
            return Err(format!("Snapshot path is not a directory: {:?}", snapshot_dir));
        }

        debug!("Valid snapshot directory: {:?}", snapshot_dir);
        let snapshot_path = snapshot_dir.join(file_name);

        if !snapshot_path.exists().await {
            error!("Snapshot file not found at path: {:?}", snapshot_path);
            return Err("Snapshot file not found".to_string());
        }

        debug!("Successfully found snapshot: {:?}", snapshot_path);
        Ok(snapshot_path)
    }

    pub async fn restore_snapshot_from_upload(
        &self,
        file_path: &PathBuf,
        original_file_name: &str,
        space_name: &str,
        version_id: i32,
    ) -> Result<(), String> {
        let final_path = Path::new(&Config::data_path()).join(&original_file_name);

        fs::rename(file_path, &final_path)
            .await
            .map_err(|e| format!("Failed to move file: {}", e))?;

        self.restore_snapshot(original_file_name)
            .map_err(|e| format!("Failed to restore snapshot: {}", e))
    }
}

impl Drop for SnapshotServiceManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_snapshot_service_manager_free(self.inner);
        }
    }
}

unsafe impl Send for SnapshotServiceManagerWrapper {}
unsafe impl Sync for SnapshotServiceManagerWrapper {}
