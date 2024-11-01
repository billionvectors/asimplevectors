use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for SearchServiceManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SearchServiceManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_search_service_manager_new() -> *mut SearchServiceManager;
    pub fn atv_search_service_manager_free(manager: *mut SearchServiceManager);
    pub fn atv_search_service_search(
        manager: *mut SearchServiceManager,
        space_name: *const c_char,
        version_unique_id: i32,
        query_json: *const c_char,
        k: usize,
    ) -> *mut c_char;
}

// Safe Rust wrapper for SearchServiceManager
#[derive(Clone, Debug)]
pub struct SearchServiceManagerWrapper {
    inner: *mut SearchServiceManager,
}

impl SearchServiceManagerWrapper {
    pub fn new() -> Self {
        unsafe { SearchServiceManagerWrapper { inner: atv_search_service_manager_new() } }
    }

    pub fn search(&self, space_name: &str, version_unique_id: i32, query_json: &str, k: usize) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        let query_json_c = CString::new(query_json).unwrap();
        unsafe {
            let result = atv_search_service_search(self.inner, space_name_c.as_ptr(), version_unique_id, query_json_c.as_ptr(), k);
            if result.is_null() {
                Err("Search failed".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for SearchServiceManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_search_service_manager_free(self.inner);
        }
    }
}

unsafe impl Send for SearchServiceManagerWrapper {}
unsafe impl Sync for SearchServiceManagerWrapper {}
