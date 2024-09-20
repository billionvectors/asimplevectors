use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// FFI declaration for RbacTokenDTOManager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct RbacTokenDTOManager {
    _private: [u8; 0],
}

extern "C" {
    pub fn atv_rbac_token_dto_manager_new() -> *mut RbacTokenDTOManager;
    pub fn atv_rbac_token_dto_manager_free(manager: *mut RbacTokenDTOManager);
    pub fn atv_rbac_token_get_system_permission(manager: *mut RbacTokenDTOManager, token: *const c_char) -> i32;
    pub fn atv_rbac_token_get_space_permission(manager: *mut RbacTokenDTOManager, token: *const c_char) -> i32;
    pub fn atv_rbac_token_get_version_permission(manager: *mut RbacTokenDTOManager, token: *const c_char) -> i32;
    pub fn atv_rbac_token_get_vector_permission(manager: *mut RbacTokenDTOManager, token: *const c_char) -> i32;
    pub fn atv_rbac_token_get_snapshot_permission(manager: *mut RbacTokenDTOManager, token: *const c_char) -> i32;
    pub fn atv_rbac_token_new_token(manager: *mut RbacTokenDTOManager, json_str: *const c_char, token: *const c_char) -> *mut c_char;
    pub fn atv_rbac_token_generate_jwt_token(manager: *mut RbacTokenDTOManager, expire_days: i32) -> *mut c_char;
    pub fn atv_rbac_token_list_tokens(manager: *mut RbacTokenDTOManager) -> *mut c_char;
    pub fn atv_rbac_token_delete_token(manager: *mut RbacTokenDTOManager, token: *const c_char);
    pub fn atv_rbac_token_update_token(manager: *mut RbacTokenDTOManager, token: *const c_char, json_str: *const c_char);
}

// Safe Rust wrapper for RbacTokenDTOManager
#[derive(Clone, Debug)]
pub struct RbacTokenDTOManagerWrapper {
    inner: *mut RbacTokenDTOManager,
}

impl RbacTokenDTOManagerWrapper {
    pub fn new() -> Self {
        unsafe { RbacTokenDTOManagerWrapper { inner: atv_rbac_token_dto_manager_new() } }
    }

    pub fn get_system_permission(&self, token: &str) -> i32 {
        let token_c = CString::new(token).unwrap();
        unsafe { atv_rbac_token_get_system_permission(self.inner, token_c.as_ptr()) }
    }

    pub fn get_space_permission(&self, token: &str) -> i32 {
        let token_c = CString::new(token).unwrap();
        unsafe { atv_rbac_token_get_space_permission(self.inner, token_c.as_ptr()) }
    }

    pub fn get_version_permission(&self, token: &str) -> i32 {
        let token_c = CString::new(token).unwrap();
        unsafe { atv_rbac_token_get_version_permission(self.inner, token_c.as_ptr()) }
    }

    pub fn get_vector_permission(&self, token: &str) -> i32 {
        let token_c = CString::new(token).unwrap();
        unsafe { atv_rbac_token_get_vector_permission(self.inner, token_c.as_ptr()) }
    }

    pub fn get_snapshot_permission(&self, token: &str) -> i32 {
        let token_c = CString::new(token).unwrap();
        unsafe { atv_rbac_token_get_snapshot_permission(self.inner, token_c.as_ptr()) }
    }

    pub fn new_token(&self, json_str: &str, token: &str) -> Result<String, String> {
        let json_str_c = CString::new(json_str).unwrap();
        let token_c = CString::new(token).unwrap();
        unsafe {
            let result = atv_rbac_token_new_token(self.inner, json_str_c.as_ptr(), token_c.as_ptr());
            if result.is_null() {
                Err("Failed to create new token".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn generate_jwt_token(&self, expire_days: i32) -> Result<String, String> {
        unsafe {
            let result = atv_rbac_token_generate_jwt_token(self.inner, expire_days);
            if result.is_null() {
                Err("Failed to generate token".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn list_tokens(&self) -> Result<String, String> {
        unsafe {
            let result = atv_rbac_token_list_tokens(self.inner);
            if result.is_null() {
                Err("Failed to list tokens".to_string())
            } else {
                let result_str = CStr::from_ptr(result).to_string_lossy().into_owned();
                super::atv_free_json_string(result);
                Ok(result_str)
            }
        }
    }

    pub fn delete_token(&self, token: &str) -> Result<(), String> {
        let token_c = CString::new(token).unwrap();
        unsafe {
            atv_rbac_token_delete_token(self.inner, token_c.as_ptr());
        };

        Ok(())
    }

    pub fn update_token(&self, token: &str, json_str: &str) -> Result<(), String> {
        let token_c = CString::new(token).unwrap();
        let json_str_c = CString::new(json_str).unwrap();
        unsafe {
            atv_rbac_token_update_token(self.inner, token_c.as_ptr(), json_str_c.as_ptr());
        };

        Ok(())
    }
}

impl Drop for RbacTokenDTOManagerWrapper {
    fn drop(&mut self) {
        unsafe {
            atv_rbac_token_dto_manager_free(self.inner);
        }
    }
}

unsafe impl Send for RbacTokenDTOManagerWrapper {}
unsafe impl Sync for RbacTokenDTOManagerWrapper {}
