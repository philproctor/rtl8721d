use crate::util::MemoryStream;

pub trait Body: bare_io::Read {
    fn content_length(&self) -> usize;
}

impl Body for &[u8] {
    fn content_length(&self) -> usize {
        self.len()
    }
}

impl Body for MemoryStream {
    fn content_length(&self) -> usize {
        self.buf_ref().len()
    }
}
