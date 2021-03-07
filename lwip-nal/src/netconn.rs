use crate::*;

pub struct Netconn(*mut c::netconn);

impl Netconn {
    pub fn new() -> Self {
        let ptr = unsafe {
            c::netconn_new_with_proto_and_callback(
                c::NETCONN_TYPE_TCP,
                c::IPPROTO_TCP,
                // Some(Self::event_handler),
                None,
            )
        };

        let mut res = Self(ptr);
        unsafe { (*res.0).flags = 0x02 };
        unsafe { (*res.0).recv_timeout = -1 };
        res
    }

    // extern "C" fn event_handler(_conn: *mut c::netconn, _event: u8, _len: u16) {
    //     let s = unsafe { (*conn).state };
    //     let state = Self::get_state_from(s);
    //     lwip_debug!("On connection: {:?} with state {:?} {}", conn, state, s);
    //     lwip_debug!("Full object: {:?}", (&*conn));
    //     match event {
    //         c::NETCONN_EVT_EVT_RCVPLUS => {
    //             lwip_debug!("Event: RCVPLUS");
    //         }
    //         c::NETCONN_EVT_EVT_RCVMINUS => {
    //             lwip_debug!("Event: RCVMINUS");
    //         }
    //         c::NETCONN_EVT_EVT_SENDPLUS => {
    //             lwip_debug!("Event: SENDPLUS");
    //         }
    //         c::NETCONN_EVT_EVT_SENDMINUS => {
    //             lwip_debug!("Event: SENDMINUS");
    //         }
    //         c::NETCONN_EVT_EVT_ERROR => {
    //             lwip_debug!("Event: ERROR");
    //         }
    //         e => {
    //             lwip_debug!("Unknown event: {}", e);
    //         }
    //     };
    // }

    pub fn from_raw_ptr(ptr: *mut c::netconn) -> Self {
        let mut res = Self(ptr);
        unsafe { (*res.0).flags = 0x02 };
        res
    }

    pub fn get_state(&self) -> TCPState {
        if self.0.is_null() {
            return TCPState::Closed;
        }
        Self::get_state_from(self.as_ref().state)
    }

    pub fn get_state_from(s: c::netconn_state) -> TCPState {
        match s {
            c::NETCONN_STATE_CONNECT => TCPState::Connected,
            c::NETCONN_STATE_NONE => TCPState::Initialized,
            c::NETCONN_STATE_WRITE => TCPState::Connected,
            c::NETCONN_STATE_LISTEN => TCPState::Listening,
            c::NETCONN_STATE_CLOSE => TCPState::Closed,
            _ => TCPState::Failed(LwipError::Unsupported),
        }
    }

    pub fn accept_pending(&self) -> bool {
        (unsafe { c::uxQueueMessagesWaiting(self.as_ref().acceptmbox) }) > 0
    }

    pub fn data_pending(&self) -> bool {
        (unsafe { c::uxQueueMessagesWaiting(self.as_ref().recvmbox) }) > 0
    }

    pub fn bind(&mut self, port: u16) -> i8 {
        if self.0.is_null() {
            return c::ERR_ARG;
        }
        unsafe { c::netconn_bind(self.0, &c::ip_addr_any, port) }
    }

    pub fn connect(&mut self, ipaddr: c::ip_addr_t, port: u16) -> i8 {
        if self.0.is_null() {
            return c::ERR_ARG;
        }
        unsafe { c::netconn_connect(self.0, &ipaddr, port) }
    }

    pub fn listen(&mut self) -> i8 {
        if self.0.is_null() {
            return c::ERR_ARG;
        }
        unsafe { c::netconn_listen_with_backlog(self.0, 0xFF) }
    }

    pub fn accept(&mut self) -> (i8, Self) {
        if self.0.is_null() {
            return (c::ERR_ARG, Self::from_raw_ptr(core::ptr::null_mut()));
        }
        let mut newconn = core::ptr::null_mut();
        let code = unsafe { c::netconn_accept(self.0, &mut newconn) };
        let mut res = Self(newconn);
        unsafe { (*res.0).flags = 0x02 };
        (code, res)
    }

    pub fn close(&mut self) -> i8 {
        if self.0.is_null() {
            return c::ERR_ARG;
        }
        unsafe { c::netconn_close(self.0) }
    }

    pub fn write(&mut self, data: &[u8]) -> (i8, u32) {
        if self.0.is_null() {
            return (c::ERR_ARG, 0);
        }
        let mut written = 0;
        let res = unsafe {
            c::netconn_write_partly(
                self.0,
                data.as_ptr().cast(),
                data.len() as u32,
                c::NETCONN_COPY,
                &mut written,
            )
        };
        (res, written)
    }

    pub fn recv(&mut self) -> (i8, Pbuf) {
        if self.0.is_null() {
            return (c::ERR_ARG, unsafe { Pbuf::empty() });
        }
        let mut buf = core::ptr::null_mut();
        let res = unsafe { c::netconn_recv_tcp_pbuf(self.0, &mut buf) };
        (res, unsafe { Pbuf::new(buf) })
    }

    pub fn pcb(&self) -> &c::tcp_pcb {
        let tcp_pcb: *const c::tcp_pcb = unsafe { ((&(*self.0).pcb) as *const c::pcb).cast() };
        unsafe { &*tcp_pcb }
    }

    pub fn as_ref(&self) -> &c::netconn {
        unsafe { &*self.0 }
    }
}

impl Drop for Netconn {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { c::netconn_close(self.0) };
            unsafe { c::netconn_delete(self.0) };
            lwip_debug!("netconn closed and freed");
        }
    }
}
