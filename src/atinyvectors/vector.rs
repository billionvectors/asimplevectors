use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for VectorServiceManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct VectorServiceManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_vector_service_manager_new() -> *mut VectorServiceManager;
    pub fn atv_vector_service_manager_free(manager: *mut VectorServiceManager);
    pub fn atv_vector_service_upsert(
        manager: *mut VectorServiceManager,
        space_name: *const c_char,
        version_id: i32,
        json_str: *const c_char,
    );
    pub fn atv_vector_service_get_vectors_by_version_id(
        manager: *mut VectorServiceManager, 
        space_name: *const c_char,
        version_id: i32, 
        start: i32, 
        limit: i32,
        filter: *const c_char,) -> *mut c_char;
}

// Safe Rust wrapper for VectorServiceManager
#[derive(Clone, Debug)]
pub struct VectorServiceManagerWrapper {
    inner: *mut VectorServiceManager,
}

impl VectorServiceManagerWrapper {
    pub fn new() -> Self {
        unsafe { VectorServiceManagerWrapper { inner: atv_vector_service_manager_new() } }
    }

    pub fn upsert_vectors(&self, space_name: &str, version_id: i32, json_str: &str) -> Result<(), String> {
        let space_name_c = CString::new(space_name).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_vector_service_upsert(self.inner, space_name_c.as_ptr(), version_id, json_str_c.as_ptr());
        };

        Ok(())
    }

    pub fn get_vectors_by_version_id(&self, space_name: &str, version_id: i32, start: i32, limit: i32, filter: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        let filter_c = CString::new(filter).unwrap();

        unsafe {
            let result = atv_vector_service_get_vectors_by_version_id(self.inner, space_name_c.as_ptr(), version_id, start, limit, filter_c.as_ptr());
            if result.is_null() {
                Err("Failed to extract vectors".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for VectorServiceManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_vector_service_manager_free(self.inner);
        }
    }
}

unsafe impl Send for VectorServiceManagerWrapper {}
unsafe impl Sync for VectorServiceManagerWrapper {}
