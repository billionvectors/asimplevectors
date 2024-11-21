use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for IdCacheManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct IdCacheManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_id_cache_manager_new() -> *mut IdCacheManager;
    pub fn atv_id_cache_manager_free(manager: *mut IdCacheManager);

    pub fn atv_id_cache_get_version_id(manager: *mut IdCacheManager, space_name: *const c_char, version_unique_id: i32) -> i32;
    pub fn atv_id_cache_get_default_version_id(manager: *mut IdCacheManager, space_name: *const c_char) -> i32;
    pub fn atv_id_cache_get_vector_index_id(manager: *mut IdCacheManager, space_name: *const c_char, version_unique_id: i32) -> i32;

    pub fn atv_id_cache_get_space_name_and_version_unique_id(manager: *mut IdCacheManager, version_id: i32) -> *mut c_char;
    pub fn atv_id_cache_get_space_name_and_version_unique_id_by_vector_index_id(manager: *mut IdCacheManager, vector_index_id: i32) -> *mut c_char;

    pub fn atv_id_cache_clean(manager: *mut IdCacheManager);
    pub fn atv_id_cache_clear_space_name_cache(manager: *mut IdCacheManager);
}

// Safe Rust wrapper for IdCacheManager
#[derive(Clone, Debug)]
pub struct IdCacheManagerWrapper {
    inner: *mut IdCacheManager,
}

impl IdCacheManagerWrapper {
    pub fn new() -> Self {
        unsafe { IdCacheManagerWrapper { inner: atv_id_cache_manager_new() } }
    }

    pub fn get_version_id(&self, space_name: &str, version_unique_id: i32) -> i32 {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe { atv_id_cache_get_version_id(self.inner, space_name_c.as_ptr(), version_unique_id) }
    }

    pub fn get_default_version_id(&self, space_name: &str) -> i32 {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe { atv_id_cache_get_default_version_id(self.inner, space_name_c.as_ptr()) }
    }

    pub fn get_vector_index_id(&self, space_name: &str, version_unique_id: i32) -> i32 {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe { atv_id_cache_get_vector_index_id(self.inner, space_name_c.as_ptr(), version_unique_id) }
    }

    pub fn get_space_name_and_version_unique_id(&self, version_id: i32) -> Result<(String, i32), String> {
        unsafe {
            let result = atv_id_cache_get_space_name_and_version_unique_id(self.inner, version_id);
            if result.is_null() {
                Err("Failed to retrieve space name and version unique ID".to_string())
            } else {
                let json_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                serde_json::from_str::<(String, i32)>(&json_str).map_err(|e| e.to_string())
            }
        }
    }

    pub fn get_space_name_and_version_unique_id_by_vector_index_id(&self, vector_index_id: i32) -> Result<(String, i32), String> {
        unsafe {
            let result = atv_id_cache_get_space_name_and_version_unique_id_by_vector_index_id(self.inner, vector_index_id);
            if result.is_null() {
                Err("Failed to retrieve space name and version unique ID by vector index ID".to_string())
            } else {
                let json_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                serde_json::from_str::<(String, i32)>(&json_str).map_err(|e| e.to_string())
            }
        }
    }

    pub fn clean(&self) {
        unsafe {
            atv_id_cache_clean(self.inner);
        }
    }

    pub fn clear_space_name_cache(&self) {
        unsafe {
            atv_id_cache_clear_space_name_cache(self.inner);
        }
    }
}

impl Drop for IdCacheManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_id_cache_manager_free(self.inner);
        }
    }
}

unsafe impl Send for IdCacheManagerWrapper {}
unsafe impl Sync for IdCacheManagerWrapper {}
