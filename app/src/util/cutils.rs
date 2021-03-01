pub use crate::prelude::*;

pub struct CString {
    inner: Vec<u8>,
}

impl CString {
    pub fn new(cin: &str) -> Self {
        let mut inner: Vec<u8> = cin.as_bytes().into();
        inner.push(0u8);
        Self { inner }
    }

    pub unsafe fn mut_ptr(&mut self) -> *mut u8 {
        &mut self.inner[0]
    }

    pub unsafe fn ptr(&self) -> *const u8 {
        &self.inner[0]
    }
}
