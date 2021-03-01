use crate::prelude::*;
use alloc::vec::Vec;
use async_trait::async_trait;

pub trait AsyncReader {
    fn try_read(&self, size: usize) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait AsyncReaderExt {
    async fn read(&self, size: usize) -> Result<Vec<u8>>;
    async fn read_until(&self, size: usize) -> Result<Vec<u8>>;
    async fn read_one(&self) -> Result<Option<u8>>;
}

#[async_trait]
impl<T: AsyncReader> AsyncReaderExt for T
where
    Self: Sync + Send,
{
    async fn read(&self, size: usize) -> Result<Vec<u8>> {
        loop {
            match self.try_read(size) {
                Err(SystemError::NotReady) => yield_now().await,
                r => return r,
            }
        }
    }

    async fn read_until(&self, size: usize) -> Result<Vec<u8>> {
        let mut remaining = size as isize;
        let mut result = Vec::with_capacity(size);
        loop {
            match self.read(remaining as usize).await {
                Ok(mut r) => {
                    remaining -= r.len() as isize;
                    result.append(&mut r);
                    if remaining <= 0 {
                        return Ok(result);
                    }
                }
                e => return e,
            }
        }
    }

    async fn read_one(&self) -> Result<Option<u8>> {
        match self.read(1).await {
            Ok(mut r) => Ok(r.pop()),
            Err(e) => Err(e),
        }
    }
}

pub trait AsyncWriter {
    fn try_write(&self, val: &[u8]) -> Result<usize>;
}

#[async_trait]
pub trait AsyncWriterExt {
    async fn write(&self, val: &[u8]) -> Result<usize>;
    async fn write_all(&self, val: &[u8]) -> Result<()>;
    async fn write_one(&self, val: &u8) -> Result<usize>;
}

#[async_trait]
impl<T: AsyncWriter> AsyncWriterExt for T
where
    Self: Sync + Send,
{
    async fn write(&self, val: &[u8]) -> Result<usize> {
        loop {
            match self.try_write(val) {
                Err(SystemError::NotReady) => yield_now().await,
                Ok(u) => return Ok(u),
                r => return r,
            }
        }
    }

    async fn write_all(&self, val: &[u8]) -> Result<()> {
        let mut pos = 0;
        loop {
            match self.write(&val[pos..]).await {
                Ok(u) => {
                    pos += u;
                    if pos >= val.len() {
                        return Ok(());
                    }
                }
                Err(other) => return Err(other),
            }
        }
    }

    async fn write_one(&self, val: &u8) -> Result<usize> {
        let val = unsafe { core::slice::from_raw_parts(val as *const u8, 1) };
        self.write(val).await
    }
}
