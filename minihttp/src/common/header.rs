use crate::{ContentType, TransferEncoding};
use alloc::string::*;
use alloc::{vec, vec::Vec};

#[derive(Debug)]
pub struct HttpHeader(String, String);

impl HttpHeader {
    pub fn new<T: Into<String>, U: Into<String>>(key: T, value: U) -> Self {
        Self(key.into(), value.into())
    }

    pub fn new_content_length(len: usize) -> Self {
        Self::new("Content-Length", len.to_string())
    }

    pub fn new_content_type(typ: ContentType) -> Self {
        Self::new("Content-Type", typ.text())
    }

    pub fn key(&self) -> &str {
        &self.0.as_str()
    }

    pub fn value(&self) -> &str {
        &self.1.as_str()
    }
}

pub type HttpHeaders = Vec<HttpHeader>;

pub trait HeaderExt {
    fn parse(hdrs: &[httparse::Header]) -> Self;
    fn content_type(&self) -> ContentType;
    fn content_length(&self) -> Option<usize>;
    fn transfer_encoding(&self) -> TransferEncoding;
    fn host(&self) -> String;
}

impl HeaderExt for Vec<HttpHeader> {
    fn content_type(&self) -> ContentType {
        for hdr in self.iter() {
            if hdr.0.as_str() == "content-type" {
                return ContentType::from(hdr.1.as_str());
            }
        }
        ContentType::Undefined
    }

    fn content_length(&self) -> Option<usize> {
        for hdr in self.iter() {
            if hdr.0.as_str() == "content-length" {
                let csz: Option<usize> = hdr.1.parse().ok();
                return csz;
            }
        }
        None
    }

    fn transfer_encoding(&self) -> TransferEncoding {
        for hdr in self.iter() {
            if hdr.0.as_str() == "transfer-encoding" {
                return TransferEncoding::from(hdr.1.as_str());
            }
        }
        TransferEncoding::Identity
    }

    fn host(&self) -> String {
        for hdr in self.iter() {
            if hdr.0.as_str() == "Host" {
                return hdr.1.clone();
            }
        }
        "localhost".into()
    }

    fn parse(hdr: &[httparse::Header]) -> Self {
        let mut result = vec![];
        for hdr in hdr.iter() {
            if hdr.name != "" {
                result.push(HttpHeader(
                    hdr.name.to_lowercase(),
                    String::from_utf8(hdr.value.to_vec()).unwrap_or_default(),
                ));
            }
        }
        result
    }
}
