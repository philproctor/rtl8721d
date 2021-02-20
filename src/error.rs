// use core::result;
use crate::hal::WifiResultCode;
use alloc::string::{FromUtf8Error, String};

pub type Result<T> = core::result::Result<T, SystemError>;

#[derive(Debug)]
pub enum SystemError {
    Wifi(WifiResultCode),
    ParseError(String),
    Unknown,
}
impl From<FromUtf8Error> for SystemError {
    fn from(_: FromUtf8Error) -> Self {
        SystemError::ParseError("Could not parse string as UTF-8!".into())
    }
}
