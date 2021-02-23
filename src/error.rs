// use core::result;
use crate::net::{NetconnResult, WifiResultCode};
use alloc::string::{FromUtf8Error, String};

pub type Result<T> = core::result::Result<T, SystemError>;

#[derive(Debug)]
pub enum SystemError {
    Wifi(WifiResultCode),
    Netconn(NetconnResult),
    ParseError(String),
    OutOfRange,
    NotReady,
    Unknown,
}
impl From<FromUtf8Error> for SystemError {
    fn from(_: FromUtf8Error) -> Self {
        SystemError::ParseError("Could not parse string as UTF-8!".into())
    }
}

unsafe impl Sync for SystemError {}
unsafe impl Send for SystemError {}
