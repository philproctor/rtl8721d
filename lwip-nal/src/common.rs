use crate::*;
pub use alloc::boxed::Box;
pub use alloc::{vec, vec::Vec};
pub use core::sync::atomic::{AtomicPtr, Ordering};

pub type LwipResult<T> = embedded_nal::nb::Result<T, LwipError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LwipError {
    Code(LwipErrCode),
    Unsupported,
    Unexpected,
    InvalidState(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TCPState {
    Initialized,
    Connecting,
    Connected,
    Bound,
    Listening,
    Closed,
    Failed(LwipError),
}

#[allow(dead_code)]
#[repr(i8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum LwipErrCode {
    Ok = c::ERR_OK,
    FailedAllocation = c::ERR_MEM,
    Buffer = c::ERR_BUF,
    Timeout = c::ERR_TIMEOUT,
    Rte = c::ERR_RTE,
    InProgress = c::ERR_INPROGRESS,
    BadValue = c::ERR_VAL,
    WouldBlock = c::ERR_WOULDBLOCK,
    InUse = c::ERR_USE,
    Already = c::ERR_ALREADY,
    IsConnected = c::ERR_ISCONN,
    Connection = c::ERR_CONN,
    Interface = c::ERR_IF,
    Aborted = c::ERR_ABRT,
    Reset = c::ERR_RST,
    Closed = c::ERR_CLSD,
    InvalidArgument = c::ERR_ARG,
}

#[macro_export]
macro_rules! nb_lwip_result {
    ($res:expr, $success:expr) => {
        match LwipErrCode::from_i8($res) {
            Some(LwipErrCode::Ok) => Ok($success),
            None => Err(nb::Error::Other(LwipError::Unexpected)),
            Some(e) => Err(nb::Error::Other(LwipError::Code(e))),
        };
    };
}

#[macro_export]
macro_rules! lwip_result {
    ($res:expr, $success:expr) => {
        match LwipErrCode::from_i8($res) {
            Some(LwipErrCode::Ok) => Ok($success),
            None => Err(LwipError::Unexpected),
            Some(e) => Err(LwipError::Code(e)),
        };
    };
}

#[macro_export]
macro_rules! ip4 {
    ($ip:expr) => {
        match $ip {
            embedded_nal::IpAddr::V4(addr) => addr.into(),
            _ => return Err(nb::Error::Other(LwipError::Unsupported)),
        };
    };
}

#[macro_export]
macro_rules! c_ip4_to_native {
    ($ipaddr:expr) => {
        #[allow(unused_unsafe)]
        embedded_nal::IpAddr::V4(unsafe { $ipaddr }.into())
    };
}

#[macro_export]
macro_rules! lwip_debug {
    ( $( $msg:tt )* ) => {
        // #[allow(unused_unsafe)]
        unsafe {
            match crate::DEBUG_FN {
                Some(debug_fn) => debug_fn(&alloc::format!($( $msg )*)),
                None => {}
            }
        }
    };
}

#[macro_export]
macro_rules! check {
    ($code:expr) => {
        match LwipErrCode::from_i8($code) {
            Some(LwipErrCode::Ok) => {}
            None => {
                return Err(nb::Error::Other(LwipError::Unexpected));
            }
            Some(e) => {
                return Err(nb::Error::Other(LwipError::Code(e)));
            }
        }
    };
}
