use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for SearchDTOManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SearchDTOManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_search_dto_manager_new() -> *mut SearchDTOManager;
    pub fn atv_search_dto_manager_free(manager: *mut SearchDTOManager);
    pub fn atv_search_dto_search(
        manager: *mut SearchDTOManager,
        space_name: *const c_char,
        version_unique_id: i32,
        query_json: *const c_char,
        k: usize,
    ) -> *mut c_char;
}

// Safe Rust wrapper for SearchDTOManager
#[derive(Clone, Debug)]
pub struct SearchDTOManagerWrapper {
    inner: *mut SearchDTOManager,
}

impl SearchDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { SearchDTOManagerWrapper { inner: atv_search_dto_manager_new() } }
    }

    pub fn search(&self, space_name: &str, version_unique_id: i32, query_json: &str, k: usize) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        let query_json_c = CString::new(query_json).unwrap();
        unsafe {
            let result = atv_search_dto_search(self.inner, space_name_c.as_ptr(), version_unique_id, query_json_c.as_ptr(), k);
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

impl Drop for SearchDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_search_dto_manager_free(self.inner);
        }
    }
}

unsafe impl Send for SearchDTOManagerWrapper {}
unsafe impl Sync for SearchDTOManagerWrapper {}
