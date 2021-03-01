use crate::{HttpHeaders, HttpMethod};

use alloc::string::String;
use alloc::vec::Vec;

/// A fully formed HTTP request
#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub headers: HttpHeaders,
    pub host: String,
    pub path: String,
    pub body: Option<Vec<u8>>,
}
