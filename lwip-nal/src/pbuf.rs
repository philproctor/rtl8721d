use crate::*;
use core::cmp::min;
use core::ptr::{copy, null_mut};

#[derive(Debug)]
pub struct Pbuf {
    pub buf: *mut c::pbuf,
    offset: u16,
}

impl Pbuf {
    pub unsafe fn empty() -> Self {
        Self {
            buf: null_mut(),
            offset: 0,
        }
    }

    pub unsafe fn new(buf: *mut c::pbuf) -> Self {
        Self { buf, offset: 0 }
    }

    pub fn consume(self) -> Vec<u8> {
        let mut buf = unsafe { &mut *self.buf };
        let sz = buf.tot_len as usize;
        let mut res = vec![0u8; sz];
        let mut pos = 0;
        loop {
            let segstart = pos as usize;
            let segend = segstart + (buf.len as usize);
            unsafe {
                core::ptr::copy(
                    buf.payload,
                    res[segstart..segend].as_mut_ptr().cast(),
                    buf.len as usize,
                )
            };
            pos += buf.len;
            if buf.len == buf.tot_len {
                break;
            }
            buf = unsafe { &mut *buf.next };
        }
        res
    }

    pub fn segment_size(&self) -> u16 {
        if self.buf.is_null() {
            0
        } else {
            unsafe { *self.buf }.len
        }
    }

    pub fn can_read(&self) -> bool {
        self.segment_size() > self.offset
    }

    pub fn link(&mut self, link: *mut c::pbuf) {
        if self.buf.is_null() {
            self.buf = link;
            self.offset = 0;
        } else {
            unsafe { c::pbuf_chain(self.buf, link) };
        }
    }

    pub fn read_to(&mut self, buffer: &mut [u8]) -> u16 {
        // pbuf_copy_partial
        if self.offset > 30000 {
            lwip_debug!("Strange offset: {}", self.offset);
        }
        let sz = min(buffer.len() as u16, self.segment_size() - self.offset);
        if sz > 0 {
            unsafe {
                copy(
                    (*self.buf).payload,
                    (buffer as *mut [u8]).cast(),
                    sz as usize,
                )
            };
            self.offset += sz;
            if self.offset >= self.segment_size() {
                let next = unsafe { (*self.buf).next };
                if !next.is_null() {
                    unsafe { c::pbuf_ref(next) }; //ref must happen before dechain to prevent tails from being deallocated
                }
                self.buf = unsafe { c::pbuf_dechain(self.buf) };
                self.offset = 0;
            }
        }
        sz
    }
}

impl Drop for Pbuf {
    fn drop(&mut self) {
        if !self.buf.is_null() {
            unsafe { c::pbuf_free(self.buf) };
            lwip_debug!("packet buffer freed");
        }
    }
}
