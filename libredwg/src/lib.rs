use std::ffi::{c_char, CStr};
use std::mem::zeroed;
use std::ops::{Deref, DerefMut};
use std::path::Path;

pub use libredwg_sys::*;

pub fn get_name(name: *const c_char) -> String {
    unsafe { CStr::from_ptr(name).to_string_lossy().to_string() }
}

pub struct DwgData {
    inner: Dwg_Data,
}

impl DwgData {
    pub fn from_dwg(path: impl AsRef<Path>) -> Self {
        let mut inner = unsafe { zeroed() };
        let path = process_file(path.as_ref());
        unsafe { dwg_read_file(path.as_ptr(), &mut inner) };
        Self { inner }
    }

    pub fn from_dxf(path: impl AsRef<Path>) -> Self {
        let mut inner = unsafe { zeroed() };
        let path = process_file(path.as_ref());
        unsafe { dxf_read_file(path.as_ptr(), &mut inner) };
        Self { inner }
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
