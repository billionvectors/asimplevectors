use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for RerankServiceManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct RerankServiceManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_rerank_service_manager_new() -> *mut RerankServiceManager;
    pub fn atv_rerank_service_manager_free(manager: *mut RerankServiceManager);
    pub fn atv_rerank_service_rerank(
        manager: *mut RerankServiceManager,
        space_name: *const c_char,
        version_unique_id: i32,
        query_json: *const c_char,
        k: usize,
    ) -> *mut c_char;
}

// Safe Rust wrapper for RerankServiceManager
#[derive(Clone, Debug)]
pub struct RerankServiceManagerWrapper {
    inner: *mut RerankServiceManager,
}

impl RerankServiceManagerWrapper {
    /// Create a new RerankServiceManager instance
    pub fn new() -> Self {
        unsafe { RerankServiceManagerWrapper { inner: atv_rerank_service_manager_new() } }
    }

    /// Perform a rerank operation
    ///
    /// # Arguments
    ///
    /// - `space_name`: The name of the space.
    /// - `version_unique_id`: The unique ID of the version.
    /// - `query_json`: The JSON query string containing the initial search and tokens for reranking.
    /// - `k`: The maximum number of results to return.
    ///
    /// # Returns
    ///
    /// `Ok(String)` with the JSON results, or `Err(String)` with an error message.
    pub fn rerank(&self, space_name: &str, version_unique_id: i32, query_json: &str, k: usize) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        let query_json_c = CString::new(query_json).unwrap();
        unsafe {
            let result = atv_rerank_service_rerank(self.inner, space_name_c.as_ptr(), version_unique_id, query_json_c.as_ptr(), k);
            if result.is_null() {
                Err("Rerank failed".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for RerankServiceManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_rerank_service_manager_free(self.inner);
        }
    }
}

unsafe impl Send for RerankServiceManagerWrapper {}
unsafe impl Sync for RerankServiceManagerWrapper {}
