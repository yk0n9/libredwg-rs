use std::ffi::CString;
use std::fs::{copy, create_dir_all, remove_dir_all};
use std::mem::zeroed;
use std::path::Path;

use nanoid::nanoid;

pub use dwg_api::*;

pub mod dwg_api;

fn process_file(path: &Path) -> CString {
    let parent = path.join("..");
    let cache = parent
        .join("cache")
        .join(Path::new(&nanoid!()).with_extension(path.extension().unwrap()));
    remove_dir_all(&cache.join("..")).ok();
    let path_string = path.to_string_lossy().to_string();
    let path = if !path_string.is_ascii() {
        create_dir_all(parent.join("cache")).ok();
        copy(&path, &cache).ok();
        cache.to_string_lossy().to_string()
    } else { path_string };
    CString::new(path).unwrap()
}

pub fn read_dwg(path: impl AsRef<Path>) -> Dwg_Data {
    let mut data = unsafe { zeroed() };
    let path = process_file(path.as_ref());
    unsafe { dwg_read_file(path.as_ptr(), &mut data) };
    data
}

pub fn read_dxf(path: impl AsRef<Path>) -> Dwg_Data {
    let mut data = unsafe { zeroed() };
    let path = process_file(path.as_ref());
    unsafe { dxf_read_file(path.as_ptr(), &mut data) };
    data
}
