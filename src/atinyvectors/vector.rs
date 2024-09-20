use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for VectorDTOManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct VectorDTOManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_vector_dto_manager_new() -> *mut VectorDTOManager;
    pub fn atv_vector_dto_manager_free(manager: *mut VectorDTOManager);
    pub fn atv_vector_dto_upsert(
        manager: *mut VectorDTOManager,
        space_name: *const c_char,
        version_id: i32,
        json_str: *const c_char,
    );
    pub fn atv_vector_dto_get_vectors_by_version_id(manager: *mut VectorDTOManager, version_id: i32) -> *mut c_char;
}

// Safe Rust wrapper for VectorDTOManager
#[derive(Clone, Debug)]
pub struct VectorDTOManagerWrapper {
    inner: *mut VectorDTOManager,
}

impl VectorDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { VectorDTOManagerWrapper { inner: atv_vector_dto_manager_new() } }
    }

    pub fn upsert_vectors(&self, space_name: &str, version_id: i32, json_str: &str) -> Result<(), String> {
        let space_name_c = CString::new(space_name).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_vector_dto_upsert(self.inner, space_name_c.as_ptr(), version_id, json_str_c.as_ptr());
        };

        Ok(())
    }

    pub fn get_vectors_by_version_id(&self, version_id: i32) -> Result<String, String> {
        unsafe {
            let result = atv_vector_dto_get_vectors_by_version_id(self.inner, version_id);
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

impl Drop for VectorDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_vector_dto_manager_free(self.inner);
        }
    }
}

unsafe impl Send for VectorDTOManagerWrapper {}
unsafe impl Sync for VectorDTOManagerWrapper {}
