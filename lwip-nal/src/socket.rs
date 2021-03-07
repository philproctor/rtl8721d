use crate::*;
use embedded_nal::{IpAddr, SocketAddr};
// use rtl8720_sys::c;

/// Wrapper around netconn objects to make them safe(r) to use
pub struct LwipSocket {
    conn: Netconn,
}

impl LwipSocket {
    pub fn new_tcp() -> Self {
        Self {
            conn: Netconn::new(),
        }
    }

    pub fn get_state(&self) -> TCPState {
        self.conn.get_state()
    }

    pub fn connect(&mut self, ipaddr: c::ip_addr_t, port: u16) -> LwipResult<()> {
        let res = self.conn.connect(ipaddr, port);
        check!(res);
        Ok(())
    }

    pub fn bind(&mut self, port: u16) -> LwipResult<()> {
        let res = self.conn.bind(port);
        check!(res);
        Ok(())
    }

    pub fn listen(&mut self) -> LwipResult<()> {
        let res = self.conn.listen();
        check!(res);
        Ok(())
    }

    pub fn accept(&mut self) -> LwipResult<Self> {
        if self.conn.accept_pending() {
            let (res, newconn) = self.conn.accept();
            check!(res);
            Ok(Self { conn: newconn })
        } else {
            Err(embedded_nal::nb::Error::WouldBlock)
        }
    }

    pub fn close(&mut self) -> LwipResult<()> {
        let res = self.conn.close();
        check!(res);
        Ok(())
    }

    pub fn send(&mut self, data: &[u8]) -> LwipResult<usize> {
        let (res, written) = self.conn.write(data);
        check!(res);
        Ok(written as usize)
    }

    pub fn recv(&mut self) -> LwipResult<Pbuf> {
        if self.conn.data_pending() {
            let (res, buf) = self.conn.recv();
            check!(res);
            Ok(buf)
        } else {
            Err(embedded_nal::nb::Error::WouldBlock)
        }
    }

    pub fn remote_ip(&self) -> IpAddr {
        c_ip4_to_native!(self.conn.pcb().remote_ip.addr)
    }

    pub fn remote_port(&self) -> u16 {
        self.conn.pcb().remote_port
    }

    pub fn remote_sockaddr(&self) -> SocketAddr {
        SocketAddr::new(self.remote_ip(), self.remote_port())
    }

    #[allow(dead_code)]
    pub fn local_ip(&self) -> IpAddr {
        c_ip4_to_native!(self.conn.pcb().local_ip.addr)
    }

    #[allow(dead_code)]
    pub fn local_port(&self) -> u16 {
        self.conn.pcb().local_port
    }

    #[allow(dead_code)]
    pub fn local_sockaddr(&self) -> SocketAddr {
        SocketAddr::new(self.local_ip(), self.local_port())
    }
}
