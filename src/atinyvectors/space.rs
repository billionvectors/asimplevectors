use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for SpaceDTOManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SpaceDTOManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_space_dto_manager_new() -> *mut SpaceDTOManager;
    pub fn atv_space_dto_manager_free(manager: *mut SpaceDTOManager);
    pub fn atv_space_dto_create_space(manager: *mut SpaceDTOManager, json_str: *const c_char);
    pub fn atv_space_dto_update_space(manager: *mut SpaceDTOManager, space_name: *const c_char, json_str: *const c_char);
    pub fn atv_space_dto_delete_space(manager: *mut SpaceDTOManager, space_name: *const c_char, json_str: *const c_char);
    pub fn atv_space_dto_get_by_space_id(manager: *mut SpaceDTOManager, space_id: i32) -> *mut c_char;
    pub fn atv_space_dto_get_by_space_name(manager: *mut SpaceDTOManager, space_name: *const c_char) -> *mut c_char;
    pub fn atv_space_dto_get_lists(manager: *mut SpaceDTOManager) -> *mut c_char;
}

// Safe Rust wrapper for SpaceDTOManager
#[derive(Clone, Debug)]
pub struct SpaceDTOManagerWrapper {
    inner: *mut SpaceDTOManager,
}

impl SpaceDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { SpaceDTOManagerWrapper { inner: atv_space_dto_manager_new() } }
    }

    pub fn create_space(&self, json_str: &str) -> Result<(), String> {
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_space_dto_create_space(self.inner, json_str_c.as_ptr());
        };

        Ok(())
    }

    pub fn get_by_space_id(&self, space_id: i32) -> Result<String, String> {
        unsafe {
            let result = atv_space_dto_get_by_space_id(self.inner, space_id);
            if result.is_null() {
                Err("Failed to get space by ID".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn update_space(&self, space_name: &str, json_str: &str) -> Result<(), String> {
        let space_name_c = CString::new(space_name).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_space_dto_update_space(self.inner, space_name_c.as_ptr(), json_str_c.as_ptr());
            Ok(())
        }
    }

    pub fn delete_space(&self, space_name: &str, json_str: &str) -> Result<(), String> {
        let space_name_c = CString::new(space_name).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_space_dto_delete_space(self.inner, space_name_c.as_ptr(), json_str_c.as_ptr());
            Ok(())
        }
    }

    pub fn get_by_space_name(&self, space_name: &str) -> Result<String, String> {
        let space_name_c = CString::new(space_name).unwrap();
        unsafe {
            let result = atv_space_dto_get_by_space_name(self.inner, space_name_c.as_ptr());
            if result.is_null() {
                Err("Failed to get space by name".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn get_lists(&self) -> Result<String, String> {
        unsafe {
            let result = atv_space_dto_get_lists(self.inner);
            if result.is_null() {
                Err("Failed to get space lists".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }
}

impl Drop for SpaceDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_space_dto_manager_free(self.inner);
        }
    }
}

unsafe impl Send for SpaceDTOManagerWrapper {}
unsafe impl Sync for SpaceDTOManagerWrapper {}
