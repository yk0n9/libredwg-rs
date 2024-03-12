use std::ffi::CString;
use std::fs::{copy, create_dir_all};
use std::path::{Path, PathBuf};

use nanoid::nanoid;

pub use dwg_api::*;

pub mod dwg_api;

pub fn process_file(path: &Path) -> (PathBuf, CString) {
    let parent = path.join("..");
    let cache = parent
        .join("cache")
        .join(Path::new(&nanoid!()).with_extension(path.extension().unwrap()));
    let path_string = path.to_string_lossy().to_string();
    let path = if !path_string.is_ascii() {
        create_dir_all(parent.join("cache")).ok();
        copy(path, &cache).ok();
        cache.to_string_lossy().to_string()
    } else { path_string };
    (cache.join(".."), CString::new(path).unwrap())
}
