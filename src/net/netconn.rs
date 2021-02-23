use crate::ffi::c;
pub use crate::prelude::*;
use num::FromPrimitive;

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum NetconnResult {
    Ok = c::err_enum_t_ERR_OK,
    FailedAllocation = c::err_enum_t_ERR_MEM,
    Buffer = c::err_enum_t_ERR_BUF,
    Timeout = c::err_enum_t_ERR_TIMEOUT,
    Rte = c::err_enum_t_ERR_RTE,
    InProgress = c::err_enum_t_ERR_INPROGRESS,
    BadValue = c::err_enum_t_ERR_VAL,
    WouldBlock = c::err_enum_t_ERR_WOULDBLOCK,
    InUse = c::err_enum_t_ERR_USE,
    Already = c::err_enum_t_ERR_ALREADY,
    IsConnected = c::err_enum_t_ERR_ISCONN,
    Connection = c::err_enum_t_ERR_CONN,
    Interface = c::err_enum_t_ERR_IF,
    Aborted = c::err_enum_t_ERR_ABRT,
    Reset = c::err_enum_t_ERR_RST,
    Closed = c::err_enum_t_ERR_CLSD,
    InvalidArgument = c::err_enum_t_ERR_ARG,
}

impl NetconnResult {
    pub fn result<T>(code: i32, ok: T) -> Result<T> {
        match Self::from_i32(code) {
            Some(Self::Ok) | Some(Self::WouldBlock) => Ok(ok),
            None => Err(SystemError::Netconn(Self::InvalidArgument)),
            Some(e) => Err(SystemError::Netconn(e)),
        }
    }
}
