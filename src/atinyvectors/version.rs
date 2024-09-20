use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for VersionDTOManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct VersionDTOManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_version_dto_manager_new() -> *mut VersionDTOManager;
    pub fn atv_version_dto_manager_free(manager: *mut VersionDTOManager);
    pub fn atv_version_dto_create_version(manager: *mut VersionDTOManager, space_name: *const c_char, json_str: *const c_char);
    pub fn atv_version_dto_get_by_version_id(manager: *mut VersionDTOManager, space_name: *const c_char, version_id: i32) -> *mut c_char;
    pub fn atv_version_dto_get_by_version_name(manager: *mut VersionDTOManager, space_name: *const c_char, version_name: *const c_char) -> *mut c_char;
    pub fn atv_version_dto_get_default_version(manager: *mut VersionDTOManager, space_name: *const c_char) -> *mut c_char;
    pub fn atv_version_dto_get_lists(manager: *mut VersionDTOManager, space_name: *const c_char) -> *mut c_char;
}

// Safe Rust wrapper for VersionDTOManager
#[derive(Clone, Debug)]
pub struct VersionDTOManagerWrapper {
    inner: *mut VersionDTOManager,
}

impl VersionDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { VersionDTOManagerWrapper { inner: atv_version_dto_manager_new() } }
    }

    pub fn create_version(&self, space_name: &str, json_str: &str) -> Result<(), String> {
        let space_name_c = CString::new(space_name).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_version_dto_create_version(self.inner, space_name_c.as_ptr(), json_str_c.as_ptr());
        };

        Ok(())
    }

    pub fn get_by_version_id(&self, space_name: &str, version_id: i32) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe {
            let result = atv_version_dto_get_by_version_id(self.inner, space_name_c.as_ptr(), version_id);
            if result.is_null() {
                Err("Failed to get version by ID".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_by_version_name(&self, space_name: &str, version_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        let version_name_c = CString::new(version_name).unwrap();
        unsafe {
            let result = atv_version_dto_get_by_version_name(self.inner, space_name_c.as_ptr(), version_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get version by name".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_default_version(&self, space_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe {
            let result = atv_version_dto_get_default_version(self.inner, space_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get default version".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_lists(&self, space_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe {
            let result = atv_version_dto_get_lists(self.inner, space_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get version lists".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for VersionDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_version_dto_manager_free(self.inner);
        }
    }
}

unsafe impl Send for VersionDTOManagerWrapper {}
unsafe impl Sync for VersionDTOManagerWrapper {}
