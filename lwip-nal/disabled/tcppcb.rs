pub use crate::*;
use rtl8720_sys::{c, ctypes::*};

#[repr(C)]
#[derive(Clone, Copy)]
pub union TcpPcbPtr {
    standard: *mut c::tcp_pcb,
    listen: *mut c::tcp_pcb_listen,
}

impl TcpPcbPtr {
    pub fn null() -> Self {
        Self {
            standard: core::ptr::null_mut(),
        }
    }
}

impl From<*mut c::tcp_pcb> for TcpPcbPtr {
    fn from(ptr: *mut c::tcp_pcb) -> Self {
        TcpPcbPtr { standard: ptr }
    }
}

impl From<*mut c::tcp_pcb_listen> for TcpPcbPtr {
    fn from(ptr: *mut c::tcp_pcb_listen) -> Self {
        TcpPcbPtr { listen: ptr }
    }
}

pub type ConnectFn = unsafe extern "C" fn(*mut core::ffi::c_void, *mut c::tcp_pcb, i8) -> i8;
pub type AcceptFn = unsafe extern "C" fn(*mut core::ffi::c_void, *mut c::tcp_pcb, i8) -> i8;
pub type RecvFn = unsafe extern "C" fn(
    *mut core::ffi::c_void,
    *mut c::tcp_pcb,
    *mut rtl8720_sys::bindings::pbuf,
    i8,
) -> i8;

pub struct TcpPcb(TcpPcbPtr, bool);

impl TcpPcb {
    pub fn new() -> Self {
        lwip_debug!("New socket created");
        Self(unsafe { c::tcp_new() }.into(), true)
    }

    pub unsafe fn new_from_ptr<T: Into<TcpPcbPtr>>(ptr: T) -> Self {
        lwip_debug!("New client connection created");
        Self(ptr.into(), true)
    }

    pub fn as_ref<T: Into<TcpPcbPtr>>(ptr: T) -> Self {
        Self(ptr.into(), false)
    }

    pub fn get_listener(&self) -> Self {
        let lst_ptr = unsafe { *self.0.standard }.listener;
        Self::as_ref(lst_ptr)
    }

    pub fn std_ptr(&self) -> *mut c::tcp_pcb {
        unsafe { self.0.standard }
    }

    pub fn listen_ptr(&self) -> *mut c::tcp_pcb_listen {
        unsafe { self.0.listen }
    }

    pub fn take(&mut self) -> Self {
        let ptr = self.0;
        self.0 = TcpPcbPtr::null();
        Self(ptr, self.1)
    }

    pub fn write(&mut self, data: &[u8]) -> i8 {
        unsafe {
            c::tcp_write(
                self.std_ptr(),
                data.as_ptr().cast(),
                data.len() as u16,
                c::TCP_WRITE_FLAG_COPY as u8,
            )
        }
    }

    pub fn connect(&mut self, ip: c::ip_addr_t, port: u16, fn_ptr: ConnectFn) -> i8 {
        unsafe { c::tcp_connect(self.std_ptr(), &ip, port, Some(fn_ptr)) }
    }

    pub fn bind(&mut self, port: u16) -> i8 {
        unsafe { c::tcp_bind(self.std_ptr(), &c::ip_addr_any, port) }
    }

    pub fn listen(&mut self) -> i8 {
        let mut err = 0;
        lwip_debug!("listen");
        self.0 =
            unsafe { c::tcp_listen_with_backlog_and_err(self.std_ptr(), 0xFF, &mut err) }.into();
        lwip_debug!("!listen");
        err
    }

    pub fn close(&mut self) -> i8 {
        unsafe { c::tcp_close(self.std_ptr()) }
    }

    pub fn set_recv_fn(&mut self, recv_fn: RecvFn) {
        unsafe { c::tcp_recv(self.std_ptr(), Some(recv_fn)) };
    }

    pub fn set_accept_fn(&mut self, accept_fn: AcceptFn) {
        unsafe { c::tcp_accept(self.std_ptr(), Some(accept_fn)) };
    }

    pub fn ptr_to_usize(cb: *mut c_void) -> usize {
        let res = unsafe { core::mem::transmute(cb) };
        lwip_debug!("ref converted to usize {}", res);
        res
    }

    pub fn usize_to_ptr<T>(cb: usize) -> &'static mut T {
        lwip_debug!("Converting usize {} to ref", cb);
        let res = unsafe { core::mem::transmute(cb) };
        res
    }

    pub fn set_callback_arg(&mut self, cb: usize) {
        lwip_debug!("set_callback_arg with val {}", cb);
        let arg = unsafe { core::mem::transmute(cb) };
        unsafe { c::tcp_arg(self.std_ptr(), arg) };
        lwip_debug!("!set_callback_arg");
    }
}

impl alloc::fmt::Debug for TcpPcb {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TcpPcb")
            // .field("x", &self.x)
            // .field("y", &self.y)
            .finish()
    }
}

impl Drop for TcpPcb {
    fn drop(&mut self) {
        if !unsafe { self.0.standard }.is_null() && self.1 {
            lwip_debug!("Closing PCB");
            unsafe { c::tcp_close(self.0.standard) };
        }
    }
}
