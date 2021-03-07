use alloc::string::{FromUtf8Error, String};
use lwip_nal::LwipError;
use rtl8720_wlan::WifiResultCode;

pub type Result<T> = core::result::Result<T, SystemError>;

#[derive(Debug)]
pub enum SystemError {
    Wifi(WifiResultCode),
    Socket(LwipError),
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
