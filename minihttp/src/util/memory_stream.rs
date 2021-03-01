use crate::*;
use alloc::boxed::Box;
use alloc::vec::Vec;
use bare_io::{Read, Write};

pub struct MemoryStream {
    pub inner: Vec<u8>,
    next: Option<Box<MemoryStream>>,
}

impl<'a> MemoryStream {
    pub fn new(initial_capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(initial_capacity),
            next: None,
        }
    }

    pub fn consume(mut self) -> Vec<u8> {
        match self.next {
            Some(ms) => self.inner.append(&mut ms.consume()),
            None => {}
        };
        self.inner.shrink_to_fit();
        self.inner
    }

    pub fn buf_ref(&'a self) -> &'a [u8] {
        &self.inner
    }

    pub fn add_link(&mut self, other: Self) {
        match self.next.as_mut() {
            Some(ms) => return ms.add_link(other),
            None => {}
        };
        self.next = Some(Box::new(other));
    }

    pub fn extend(&mut self, mut reader: impl bare_io::Read) -> bare_io::Result<()> {
        let mut buf = [0u8; 20];
        loop {
            let read = reader.read(&mut buf)?;
            if read > 0 {
                self.write(&buf[..read])?;
            } else {
                break;
            }
        }
        Ok(())
    }
}

impl Write for MemoryStream {
    fn write(&mut self, buf: &[u8]) -> bare_io::Result<usize> {
        self.inner.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> bare_io::Result<()> {
        Ok(())
    }
}

impl Read for MemoryStream {
    fn read(&mut self, buf: &mut [u8]) -> bare_io::Result<usize> {
        if self.inner.len() == 0 {
            if let Some(ref mut next) = self.next {
                next.read(buf)
            } else {
                Ok(0)
            }
        } else {
            let mut sz = buf.len();
            if self.inner.len() < sz {
                sz = self.inner.len();
            }
            buf[..sz].copy_from_slice(&self.inner[..sz]);
            self.inner.drain(..sz).into_iter();
            Ok(sz)
        }
    }
}
