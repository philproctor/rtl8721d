#![no_std]

extern crate alloc;

#[macro_use]
extern crate num_derive;

use alloc::{vec, vec::Vec};
pub use embedded_nal::{Dns, IpAddr, Ipv4Addr, SocketAddr, TcpClientStack, TcpFullStack};
pub use no_std_net as net;
use num_traits::FromPrimitive;

use rtl8720_sys::c;

macro_rules! ip4 {
    ($ip:expr) => {
        match $ip {
            embedded_nal::IpAddr::V4(addr) => addr.into(),
            _ => return Err(nb::Error::Other(LwipError::Unsupported)),
        };
    };
}

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

macro_rules! nb_result {
    ($res:expr, $success:expr) => {
        match NetconnResult::from_i32($res as i32) {
            Some(NetconnResult::Ok) => Ok($success),
            None => Err(nb::Error::Other(LwipError::Unexpected)),
            Some(e) => Err(nb::Error::Other(LwipError::Netconn(e))),
        };
    };
}

macro_rules! result {
    ($res:expr, $success:expr) => {
        match NetconnResult::from_i32($res as i32) {
            Some(NetconnResult::Ok) => Ok($success),
            None => Err(LwipError::Unexpected),
            Some(e) => Err(LwipError::Netconn(e)),
        };
    };
}

#[derive(Debug)]
pub enum LwipError {
    Netconn(NetconnResult),
    Unsupported,
    Unexpected,
}

pub struct LwipInterface {
    _phantom: core::marker::PhantomData<()>,
}

pub struct LwipTcpSocket {
    conn: *mut c::netconn,
    connected: bool,
    unread: Vec<u8>,
}

impl LwipInterface {
    pub fn new() -> Self {
        unsafe {
            c::LwIP_Init();
        }
        Self {
            _phantom: core::marker::PhantomData::<()>,
        }
    }

    pub fn dhcp(idx: u8) {
        unsafe { c::LwIP_DHCP(idx, c::DHCP_State_TypeDef_DHCP_START as u8) };
    }

    pub fn get_ip(idx: u8) -> IpAddr {
        let mut ip = [0u8; 4];
        unsafe {
            let ip_ptr = c::LwIP_GetIP_Idx(idx);
            ip.copy_from_slice(core::slice::from_raw_parts(ip_ptr, 4));
        }
        ip.into()
    }
}

impl Dns for LwipInterface {
    type Error = LwipError;

    fn get_host_by_name(
        &self,
        hostname: &str,
        addr_type: embedded_nal::AddrType,
    ) -> Result<IpAddr, Self::Error> {
        if addr_type == embedded_nal::AddrType::IPv6 {
            return Err(LwipError::Unsupported);
        }
        let c_hostname = cstr_core::CString::new(hostname).unwrap_or_default();
        let mut ip = [0u8; 4];
        unsafe {
            let r = c::netconn_gethostbyname(c_hostname.as_ptr().cast(), (&mut ip as *mut [u8; 4]).cast());
            result!(r, IpAddr::V4(Ipv4Addr::from(ip)))
        }
    }

    fn get_host_by_address(
        &self,
        _: IpAddr,
    ) -> Result<embedded_nal::heapless::String<heapless::consts::U256>, Self::Error> {
        unimplemented!()
    }
}

impl TcpClientStack for LwipInterface {
    type TcpSocket = LwipTcpSocket;

    type Error = LwipError;

    fn socket(&mut self) -> Result<Self::TcpSocket, Self::Error> {
        let conn = unsafe {
            c::netconn_new_with_proto_and_callback(
                c::netconn_type_NETCONN_TCP,
                c::IPPROTO_TCP as u8,
                None,
            )
        };
        Ok(LwipTcpSocket {
            conn,
            connected: false,
            unread: vec![],
        })
    }

    fn connect(
        &mut self,
        socket: &mut Self::TcpSocket,
        remote: embedded_nal::SocketAddr,
    ) -> nb::Result<(), Self::Error> {
        let ip: u32 = ip4!(remote.ip());
        let ipaddr = c::ip4_addr { addr: ip };
        nb_result!(
            unsafe { c::netconn_connect(socket.conn, &ipaddr, remote.port()) },
            ()
        )
    }

    fn is_connected(&mut self, socket: &Self::TcpSocket) -> Result<bool, Self::Error> {
        Ok(socket.connected)
    }

    fn send(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &[u8],
    ) -> nb::Result<usize, Self::Error> {
        let mut written = 0;
        let res = unsafe {
            c::netconn_write_partly(
                socket.conn,
                buffer.as_ptr().cast(),
                buffer.len() as u32,
                c::NETCONN_COPY as u8,
                &mut written,
            )
        };
        nb_result!(res, written as usize)
    }

    fn receive(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &mut [u8],
    ) -> nb::Result<usize, Self::Error> {
        if socket.unread.is_empty() {
            let mut inbuf: Vec<u8>;
            let mut head = None;
            let res;
            unsafe {
                let mut buf = core::ptr::null_mut();
                res = c::netconn_recv_tcp_pbuf(socket.conn, &mut buf);
                let mut buf_d = *buf;
                let sz = buf_d.tot_len as usize;
                inbuf = vec![0u8; sz];
                let mut pos = 0;
                loop {
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
            let head: Vec<u8> = inbuf.drain(..buffer.len()).collect();
            let sz = head.len();
            buffer[..head.len()].copy_from_slice(head.as_slice());
            if inbuf.len() > 0 {
                socket.unread.append(&mut inbuf);
            }
            nb_result!(res as i32, sz)
        } else {
            let head: Vec<u8> = socket.unread.drain(..buffer.len()).collect();
            let sz = head.len();
            buffer[..head.len()].copy_from_slice(head.as_slice());
            Ok(sz)
        }
    }

    fn close(&mut self, mut socket: Self::TcpSocket) -> Result<(), Self::Error> {
        socket.connected = false;
        result!(unsafe { c::netconn_close(socket.conn) }, ())
    }
}

impl TcpFullStack for LwipInterface {
    fn bind(&mut self, socket: &mut Self::TcpSocket, local_port: u16) -> Result<(), Self::Error> {
        result!(
            unsafe { c::netconn_bind(socket.conn, &c::ip_addr_any, local_port) },
            ()
        )
    }

    fn listen(&mut self, socket: &mut Self::TcpSocket) -> Result<(), Self::Error> {
        result!(
            unsafe { c::netconn_listen_with_backlog(socket.conn, 0) },
            ()
        )
    }

    fn accept(
        &mut self,
        socket: &mut Self::TcpSocket,
    ) -> nb::Result<(Self::TcpSocket, embedded_nal::SocketAddr), Self::Error> {
        let newconn = core::ptr::null_mut();
        result!(
            unsafe { c::netconn_accept(socket.conn, newconn) } as i32,
            ()
        )?;
        let tcp_pcb: *const c::tcp_pcb =
            unsafe { ((&(**newconn).pcb) as *const c::netconn__bindgen_ty_1).cast() };
        let port = unsafe { (*tcp_pcb).remote_port };
        let remote_ip: *const u32 = unsafe { (&(*tcp_pcb).remote_ip as *const c::ip4_addr).cast() };
        let remote_ip: u32 = unsafe { *remote_ip };
        let socket = SocketAddr::new(IpAddr::V4(remote_ip.into()), port);
        Ok((
            LwipTcpSocket {
                conn: unsafe { *newconn },
                connected: false,
                unread: vec![],
            },
            socket,
        ))
    }
}
