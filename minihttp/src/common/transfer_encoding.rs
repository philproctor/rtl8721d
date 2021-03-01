#[derive(Debug, PartialEq)]
pub enum TransferEncoding {
    Identity,
    Chunked,
    Compressed,
    Deflate,
    Gzip,
}

impl From<&str> for TransferEncoding {
    fn from(enc: &str) -> Self {
        match enc.to_lowercase().as_str() {
            "chunked" => Self::Chunked,
            "compress" => Self::Compressed,
            "deflate" => Self::Deflate,
            "gzip" => Self::Gzip,
            _ => Self::Identity,
        }
    }
}

impl Default for TransferEncoding {
    fn default() -> Self {
        Self::Identity
    }
}
