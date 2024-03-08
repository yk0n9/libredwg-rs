use std::ffi::{c_char, CStr};

pub use libredwg_sys::*;

fn get_name(name: *const c_char) -> String {
    unsafe { CStr::from_ptr(name).to_string_lossy().to_string() }
}
