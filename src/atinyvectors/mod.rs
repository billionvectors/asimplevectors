#![allow(clippy::uninlined_format_args)]
#![deny(unused_qualifications)]

pub mod atinyvectors_bo;
pub mod atinyvectors_raft_command;

pub mod rbac_token;
pub mod search;
pub mod space;
pub mod version;
pub mod vector;
pub mod snapshot;

use std::os::raw::c_char;

extern "C" {
    pub fn atv_init();
    pub fn atv_free_json_string(json_str: *mut c_char);
}
