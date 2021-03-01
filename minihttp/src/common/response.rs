use super::HttpHeaders;
use crate::HttpStatus;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HttpHeaders,
    pub body: Option<Vec<u8>>,
}
