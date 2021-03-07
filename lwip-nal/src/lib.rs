#![no_std]

extern crate alloc;

#[macro_use]
extern crate num_derive;

mod pbuf;
pub use pbuf::Pbuf;

pub mod bindings;
pub use bindings as c;

mod common;
pub use common::*;

mod socket;
pub use socket::*;

mod netconn;
pub use netconn::*;

use alloc::{vec, vec::Vec};
pub use embedded_nal::{Dns, IpAddr, Ipv4Addr, SocketAddr, TcpClientStack, TcpFullStack};
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy)]
pub struct LwipInterface;

pub struct LwipTcpSocket {
    conn: LwipSocket,
    // buf: Pbuf,
    buf: Vec<u8>,
}

static mut DEBUG_FN: Option<fn(&str)> = None;

impl LwipInterface {
    pub fn init(debug: Option<fn(&str)>) {
        unsafe {
            c::LwIP_Init();
        }
        unsafe { DEBUG_FN = debug };
    }

    pub fn dhcp(idx: u8) {
        unsafe { c::LwIP_DHCP(idx, c::DHCP_START) };
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
            let r = c::netconn_gethostbyname(
                c_hostname.as_ptr().cast(),
                (&mut ip as *mut [u8; 4]).cast(),
            );
            lwip_result!(r, IpAddr::V4(Ipv4Addr::from(ip)))
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
        Ok(LwipTcpSocket {
            //pcb v
            // conn: LwipSocket::new(),

            //netconn v
            conn: LwipSocket::new_tcp(),
            // buf: unsafe { Pbuf::empty() },
            buf: vec![],
        })
    }

    fn connect(
        &mut self,
        socket: &mut Self::TcpSocket,
        remote: embedded_nal::SocketAddr,
    ) -> nb::Result<(), Self::Error> {
        let ip: u32 = ip4!(remote.ip());
        let ipaddr = c::ip_addr_t { addr: ip };
        socket.conn.connect(ipaddr, remote.port())
    }

    fn is_connected(&mut self, socket: &Self::TcpSocket) -> Result<bool, Self::Error> {
        Ok(socket.conn.get_state() == TCPState::Connected)
    }

    fn send(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &[u8],
    ) -> nb::Result<usize, Self::Error> {
        socket.conn.send(buffer)
    }

    fn receive(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &mut [u8],
    ) -> nb::Result<usize, Self::Error> {
        if socket.buf.len() == 0 {
            let rcvd = socket.conn.recv()?;
            socket.buf = rcvd.consume();
        };
        let sz = core::cmp::min(socket.buf.len(), buffer.len());
        if sz > 0 {
            socket
                .buf
                .drain(..sz)
                .enumerate()
                .for_each(|(i, u)| buffer[i] = u);
        }
        Ok(sz as usize)
        // let rcvd = socket.conn.recv(buffer)?;
        // if rcvd == 0 {
        //     Err(nb::Error::WouldBlock)
        // } else {
        //     Ok(rcvd as usize)
        // }
    }

    fn close(&mut self, mut socket: Self::TcpSocket) -> Result<(), Self::Error> {
        nb::block!(socket.conn.close())
    }
}

impl TcpFullStack for LwipInterface {
    fn bind(&mut self, socket: &mut Self::TcpSocket, local_port: u16) -> Result<(), Self::Error> {
        nb::block!(socket.conn.bind(local_port))
    }

    fn listen(&mut self, socket: &mut Self::TcpSocket) -> Result<(), Self::Error> {
        nb::block!(socket.conn.listen())
    }

    fn accept(
        &mut self,
        socket: &mut Self::TcpSocket,
    ) -> nb::Result<(Self::TcpSocket, embedded_nal::SocketAddr), Self::Error> {
        match socket.conn.accept() {
            Ok(conn) => {
                let rsock = conn.remote_sockaddr();
                Ok((
                    LwipTcpSocket {
                        conn,
                        // buf: unsafe { Pbuf::empty() },
                        buf: vec![],
                    },
                    rsock,
                ))
            }
            // Ok(None) => Err(nb::Error::WouldBlock),
            Err(e) => Err(e),
        }
    }
}
