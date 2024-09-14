use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Forward declaration of the DTO Managers in the Rust FFI
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SearchDTOManager {
    _private: [u8; 0],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SpaceDTOManager {
    _private: [u8; 0],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VersionDTOManager {
    _private: [u8; 0],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VectorDTOManager {
    _private: [u8; 0],
}

extern "C" {
    // C API for SearchDTOManager
    pub fn search_dto_manager_new() -> *mut SearchDTOManager;
    pub fn search_dto_manager_free(manager: *mut SearchDTOManager);
    pub fn search_dto_search(
        manager: *mut SearchDTOManager,
        space_name: *const c_char,
        query_json: *const c_char,
        k: usize,
    ) -> *mut c_char;

    // C API for SpaceDTOManager
    pub fn space_dto_manager_new() -> *mut SpaceDTOManager;
    pub fn space_dto_manager_free(manager: *mut SpaceDTOManager);
    pub fn space_dto_create_space(manager: *mut SpaceDTOManager, json_str: *const c_char);
    pub fn space_dto_get_by_space_id(manager: *mut SpaceDTOManager, space_id: i32) -> *mut c_char;
    pub fn space_dto_get_by_space_name(manager: *mut SpaceDTOManager, space_name: *const c_char) -> *mut c_char;
    pub fn space_dto_get_lists(manager: *mut SpaceDTOManager) -> *mut c_char;

    // C API for VersionDTOManager
    pub fn version_dto_manager_new() -> *mut VersionDTOManager;
    pub fn version_dto_manager_free(manager: *mut VersionDTOManager);
    pub fn version_dto_create_version(manager: *mut VersionDTOManager, space_name: *const c_char, json_str: *const c_char);
    pub fn version_dto_get_by_version_id(manager: *mut VersionDTOManager, space_name: *const c_char, version_id: i32) -> *mut c_char;
    pub fn version_dto_get_by_version_name(manager: *mut VersionDTOManager, space_name: *const c_char, version_name: *const c_char) -> *mut c_char;
    pub fn version_dto_get_default_version(manager: *mut VersionDTOManager, space_name: *const c_char) -> *mut c_char;
    pub fn version_dto_get_lists(manager: *mut VersionDTOManager, space_name: *const c_char) -> *mut c_char;

    // C API for VectorDTOManager
    pub fn vector_dto_manager_new() -> *mut VectorDTOManager;
    pub fn vector_dto_manager_free(manager: *mut VectorDTOManager);
    pub fn vector_dto_upsert(
        manager: *mut VectorDTOManager,
        space_name: *const c_char,
        version_id: i32,
        json_str: *const c_char,
    );
    pub fn vector_dto_get_vectors_by_version_id(manager: *mut VectorDTOManager, version_id: i32) -> *mut c_char;

    // Function to free the JSON string returned by the C API
    pub fn free_json_string(json_str: *mut c_char);
}

// Safe Rust wrapper for SearchDTOManager
#[derive(Clone, Debug)]
pub struct SearchDTOManagerWrapper {
    inner: *mut SearchDTOManager,
}

impl SearchDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { SearchDTOManagerWrapper { inner: search_dto_manager_new() } }
    }

    pub fn search(&self, space_name: &str, query_json: &str, k: usize) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        let query_json_c = CString::new(query_json).unwrap();
        unsafe {
            let result = search_dto_search(self.inner, space_name_c.as_ptr(), query_json_c.as_ptr(), k);
            if result.is_null() {
                Err("Search failed".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for SearchDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            search_dto_manager_free(self.inner);
        }
    }
}

// Safe Rust wrapper for SpaceDTOManager
#[derive(Clone, Debug)]
pub struct SpaceDTOManagerWrapper {
    inner: *mut SpaceDTOManager,
}

impl SpaceDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { SpaceDTOManagerWrapper { inner: space_dto_manager_new() } }
    }

    pub fn create_space(&self, json_str: &str) {
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            space_dto_create_space(self.inner, json_str_c.as_ptr());
        }
    }

    pub fn get_by_space_id(&self, space_id: i32) -> Result<String, String> {
        unsafe {
            let result = space_dto_get_by_space_id(self.inner, space_id);
            if result.is_null() {
                Err("Failed to get space by ID".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_by_space_name(&self, space_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe {
            let result = space_dto_get_by_space_name(self.inner, space_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get space by name".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_lists(&self) -> Result<String, String> {
        unsafe {
            let result = space_dto_get_lists(self.inner);
            if result.is_null() {
                Err("Failed to get space lists".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for SpaceDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            space_dto_manager_free(self.inner);
        }
    }
}

// Safe Rust wrapper for VersionDTOManager
#[derive(Clone, Debug)]
pub struct VersionDTOManagerWrapper {
    inner: *mut VersionDTOManager,
}

impl VersionDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { VersionDTOManagerWrapper { inner: version_dto_manager_new() } }
    }

    pub fn create_version(&self, space_name: &str, json_str: &str) {
        let space_name_c = CString::new(space_name).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            version_dto_create_version(self.inner, space_name_c.as_ptr(), json_str_c.as_ptr());
        }
    }

    pub fn get_by_version_id(&self, space_name: &str, version_id: i32) -> Result<String, String> {
        unsafe {
            let space_name_c = CString::new(space_name).unwrap();
            let result = version_dto_get_by_version_id(self.inner, space_name_c.as_ptr(), version_id);
            if result.is_null() {
                Err("Failed to get version by ID".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_by_version_name(&self, space_name: &str, version_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        let version_name_c = CString::new(version_name).unwrap();
        unsafe {
            let result = version_dto_get_by_version_name(self.inner, space_name_c.as_ptr(), version_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get version by name".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_default_version(&self, space_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe {
            let result = version_dto_get_default_version(self.inner, space_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get default version".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_lists(&self, space_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe {
            let result = version_dto_get_lists(self.inner, space_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get version lists".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for VersionDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            version_dto_manager_free(self.inner);
        }
    }
}

// Safe Rust wrapper for VectorDTOManager
#[derive(Clone, Debug)]
pub struct VectorDTOManagerWrapper {
    inner: *mut VectorDTOManager,
}

impl VectorDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { VectorDTOManagerWrapper { inner: vector_dto_manager_new() } }
    }

    pub fn upsert_vectors(&self, space_name: &str, version_id: i32, json_str: &str) {
        let space_name_c = CString::new(space_name).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            vector_dto_upsert(self.inner, space_name_c.as_ptr(), version_id, json_str_c.as_ptr());
        }
    }

    pub fn get_vectors_by_version_id(&self, version_id: i32) -> Result<String, String> {
        unsafe {
            let result = vector_dto_get_vectors_by_version_id(self.inner, version_id);
            if result.is_null() {
                Err("Failed to extract vectors".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for VectorDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            vector_dto_manager_free(self.inner);
        }
    }
}

// Implement Send and Sync for DTO Managers
unsafe impl Send for SearchDTOManagerWrapper {}
unsafe impl Sync for SearchDTOManagerWrapper {}

unsafe impl Send for SpaceDTOManagerWrapper {}
unsafe impl Sync for SpaceDTOManagerWrapper {}

unsafe impl Send for VersionDTOManagerWrapper {}
unsafe impl Sync for VersionDTOManagerWrapper {}

unsafe impl Send for VectorDTOManagerWrapper {}
unsafe impl Sync for VectorDTOManagerWrapper {}
