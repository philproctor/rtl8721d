pub use crate::ffi::c;
pub use crate::prelude::*;

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
enum NetconnEvents {
    Received = c::netconn_evt_NETCONN_EVT_RCVPLUS,
    NegReceived = c::netconn_evt_NETCONN_EVT_RCVMINUS,
    Sent = c::netconn_evt_NETCONN_EVT_SENDPLUS,
    NegSent = c::netconn_evt_NETCONN_EVT_SENDMINUS,
    Error = c::netconn_evt_NETCONN_EVT_ERROR,
}

// extern const ip_addr_t ip_addr_any;
// c::ip_addr_any
pub enum TCP {
    Client { conn: *mut c::netconn },
    Server { conn: *mut c::netconn },
    Accepted { conn: *mut c::netconn },
}

unsafe impl Sync for TCP {}
unsafe impl Send for TCP {}

impl crate::futures::AsyncReader for TCP {
    fn try_read(&self, _: usize) -> Result<Vec<u8>> {
        let mut inbuf: Vec<u8>;
        let mut head = None;
        let res;
        unsafe {
            let mut buf = core::ptr::null_mut();
            res = c::netconn_recv_tcp_pbuf(self.conn(), &mut buf);
            let mut buf_d = *buf;
            let sz = buf_d.tot_len as usize;
            inbuf = vec![0u8; sz];
            let mut pos = 0;
            loop {
                debug!("Position: {}", pos);
                debug!("{:?} => {:?}", res, *buf);
                let part_sz = buf_d.len as usize;
                core::ptr::copy(
                    buf_d.payload,
                    ((&mut inbuf[pos]) as *mut u8) as *mut core::ffi::c_void,
                    part_sz,
                );
                pos += part_sz;
                if buf_d.len == buf_d.tot_len {
                    break;
                } else {
                    if head.is_none() {
                        head = Some(buf);
                    }
                    buf_d = *buf_d.next;
                    buf = &mut buf_d;
                }
            }
            if let Some(head) = head {
                c::pbuf_free(head);
            }
        }
        NetconnResult::result(res as i32, inbuf)
    }
}

impl AsyncWriter for TCP {
    fn try_write(&self, val: &[u8]) -> Result<usize> {
        let mut written = 0;
        let res = unsafe {
            c::netconn_write_partly(
                self.conn(),
                core::mem::transmute(val.as_ptr()),
                val.len() as u32,
                c::NETCONN_COPY as u8,
                &mut written,
            )
        };
        return NetconnResult::result(res as i32, written as usize);
    }
}

impl TCP {
    pub async fn connect(host: Host, port: u16) -> Result<Self> {
        let conn = unsafe {
            c::netconn_new_with_proto_and_callback(
                c::netconn_type_NETCONN_TCP,
                c::IPPROTO_TCP as u8,
                None,
            )
        };
        let zelf = Self::Client { conn };
        let addr = host.ip4_addr();
        let res = unsafe { c::netconn_connect(zelf.conn(), &addr, port) };
        debug!("Connect result: {}", res);
        NetconnResult::result(res as i32, zelf)
    }

    pub async fn serve(port: u16) -> Result<Self> {
        let conn = unsafe {
            c::netconn_new_with_proto_and_callback(
                c::netconn_type_NETCONN_TCP,
                c::IPPROTO_TCP as u8,
                None,
            )
        };
        NetconnResult::result(
            unsafe { c::netconn_bind(conn, &c::ip_addr_any, port) } as i32,
            (),
        )?;
        NetconnResult::result(
            unsafe { c::netconn_listen_with_backlog(conn, 0) } as i32,
            (),
        )?;

        Ok(Self::Server { conn })
    }

    pub async fn accept(&self) -> Result<Self> {
        if let Self::Server { conn } = self {
            let newconn = core::ptr::null_mut();
            NetconnResult::result(unsafe { c::netconn_accept(*conn, newconn) } as i32, ())?;
            Ok(Self::Accepted {
                conn: unsafe { *newconn },
            })
        } else {
            return Err(SystemError::Netconn(NetconnResult::BadValue));
        }
    }

    const fn conn(&self) -> *mut c::netconn {
        match *self {
            Self::Server { conn, .. } | Self::Client { conn, .. } | Self::Accepted { conn, .. } => {
                conn
            }
        }
    }
}

impl Drop for TCP {
    fn drop(&mut self) {
        unsafe {
            c::netconn_close(self.conn());
            c::netconn_delete(self.conn());
        }
    }
}
