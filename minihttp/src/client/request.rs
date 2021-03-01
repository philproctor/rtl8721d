use super::*;
use alloc::vec::Vec;

pub struct OutgoingRequest {
    pub method: HttpMethod,
    pub host: String,
    pub path: String,
    pub body: Option<Vec<u8>>,
}

impl OutgoingRequest {
    pub fn new(host: String, method: HttpMethod, path: String, body: Option<Vec<u8>>) -> Self {
        Self {
            host,
            method,
            path,
            body,
        }
    }

    pub fn consume(self) -> Vec<u8> {
        // Example request: "GET /x HTTP/1.1\r\nHost: test\r\n\r\n";
        self.method
            .text()
            .as_bytes()
            .iter()
            .cloned()
            .chain(b" ".iter().cloned())
            .chain(self.path.into_bytes())
            .chain(b" HTTP/1.1\r\nHost: ".iter().cloned())
            .chain(self.host.into_bytes())
            .chain(b"\r\nUser-Agent: minihttp\r\n\r\n".iter().cloned())
            .collect()
    }
}
