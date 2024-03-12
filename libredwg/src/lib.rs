use std::ffi::{c_char, CString};
use std::mem::zeroed;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub use libredwg_sys::*;

pub fn get_name(name: *const c_char) -> String {
    unsafe { CString::from_raw(name as *mut c_char).to_string_lossy().to_string() }
}

pub struct DwgData {
    inner: Dwg_Data,
    cache: PathBuf,
}

impl DwgData {
    pub fn from_dwg(path: impl AsRef<Path>) -> Self {
        let mut inner = unsafe { zeroed() };
        let (cache, path) = process_file(path.as_ref());
        unsafe { dwg_read_file(path.as_ptr(), &mut inner) };
        Self { inner, cache }
    }

    pub fn from_dxf(path: impl AsRef<Path>) -> Self {
        let mut inner = unsafe { zeroed() };
        let (cache, path) = process_file(path.as_ref());
        unsafe { dxf_read_file(path.as_ptr(), &mut inner) };
        Self { inner, cache }
    }

    pub fn objects(&self) -> impl Iterator<Item=Dwg_Object> {
        Iter {
            current: 0,
            len: self.num_objects as isize,
            ptr: self.object,
        }
    }
}

impl Deref for DwgData {
    type Target = Dwg_Data;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DwgData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Drop for DwgData {
    fn drop(&mut self) {
        unsafe { dwg_free(&mut self.inner); }
        std::fs::remove_dir_all(&self.cache).ok();
    }
}

pub struct Iter {
    current: isize,
    len: isize,
    ptr: *mut Dwg_Object,
}

impl Iterator for Iter {
    type Item = Dwg_Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.len {
            let i = self.current;
            self.current += 1;
            unsafe { Some(*self.ptr.offset(i)) }
        } else {
            None
        }
    }
}
