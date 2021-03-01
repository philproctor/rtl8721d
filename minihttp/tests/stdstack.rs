use anyhow::anyhow;
use embedded_nal::{AddrType, Dns, IpAddr, TcpClientStack, TcpFullStack};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::{io::prelude::*, net::TcpListener};

// This is a quick, dirty, nasty implementation of embedded_nal for std::net
// The purpose is to be able to test network ops without requiring a device

#[derive(Debug, Clone)]
pub struct TestStack;
#[derive(Debug)]
pub struct TestTCPSocket {
    connected: bool,
    stream: Option<TcpStream>,
    listener: Option<TcpListener>,
}

impl Dns for TestStack {
    type Error = anyhow::Error;

    fn get_host_by_name(&self, hostname: &str, addr_type: AddrType) -> Result<IpAddr, Self::Error> {
        let with_fake_port = if hostname.find(':').is_some() {
            format!("[{}]:1234", hostname)
        } else {
            format!("{}:1234", hostname)
        };

        let accept_v4 = addr_type != AddrType::IPv6;
        let accept_v6 = addr_type != AddrType::IPv4;

        for addr in with_fake_port.to_socket_addrs()? {
            match addr {
                SocketAddr::V4(v) if accept_v4 => {
                    return Ok(v.ip().octets().into());
                }
                SocketAddr::V6(v) if accept_v6 => {
                    return Ok(v.ip().octets().into());
                }
                _ => continue,
            }
        }

        Err(anyhow!("Not found!"))
    }

    fn get_host_by_address(
        &self,
        _: IpAddr,
    ) -> Result<embedded_nal::heapless::String<heapless::consts::U256>, Self::Error> {
        Err(anyhow!("Not found!"))
    }
}

macro_rules! ip4 {
    ($ip:expr) => {
        match $ip {
            embedded_nal::IpAddr::V4(addr) => addr.into(),
            _ => panic!("Couldn't parse ip"),
        };
    };
}

impl TcpClientStack for TestStack {
    type TcpSocket = TestTCPSocket;

    type Error = anyhow::Error;

    fn socket(&mut self) -> Result<Self::TcpSocket, Self::Error> {
        Ok(TestTCPSocket {
            connected: false,
            stream: None,
            listener: None,
        })
    }

    fn connect(
        &mut self,
        socket: &mut Self::TcpSocket,
        remote: embedded_nal::SocketAddr,
    ) -> nb::Result<(), Self::Error> {
        let ip: u32 = ip4!(remote.ip());
        let saddr = SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::from(ip)),
            remote.port(),
        );
        let sock = TcpStream::connect(saddr).unwrap();
        // sock.set_nonblocking(true).unwrap();
        socket.connected = true;
        socket.stream = Some(sock);
        Ok(())
    }

    fn is_connected(&mut self, socket: &Self::TcpSocket) -> Result<bool, Self::Error> {
        Ok(socket.connected)
    }

    fn send(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &[u8],
    ) -> nb::Result<usize, Self::Error> {
        Ok(socket.stream.as_mut().unwrap().write(buffer).unwrap())
    }

    fn receive(
        &mut self,
        socket: &mut Self::TcpSocket,
        buffer: &mut [u8],
    ) -> nb::Result<usize, Self::Error> {
        Ok(socket.stream.as_mut().unwrap().read(buffer).unwrap())
    }

    fn close(&mut self, _: Self::TcpSocket) -> Result<(), Self::Error> {
        Ok(())
    }
}

macro_rules! std_ip4 {
    ($ip:expr) => {
        match $ip {
            std::net::SocketAddr::V4(addr) => addr.ip().octets(),
            _ => panic!("Couldn't parse ip"),
        };
    };
}

impl TcpFullStack for TestStack {
    fn bind(&mut self, socket: &mut Self::TcpSocket, local_port: u16) -> Result<(), Self::Error> {
        let new_listener = TcpListener::bind(format!("127.0.0.1:{}", local_port)).unwrap();
        socket.listener = Some(new_listener);
        Ok(())
    }

    fn listen(&mut self, _: &mut Self::TcpSocket) -> Result<(), Self::Error> {
        Ok(())
    }

    fn accept(
        &mut self,
        socket: &mut Self::TcpSocket,
    ) -> nb::Result<(Self::TcpSocket, embedded_nal::SocketAddr), Self::Error> {
        let (new_client, sockaddr) = socket.listener.as_mut().unwrap().accept().unwrap();
        let new_sock = TestTCPSocket {
            connected: true,
            stream: Some(new_client),
            listener: None,
        };
        Ok((
            new_sock,
            embedded_nal::SocketAddr::new(
                embedded_nal::IpAddr::V4(embedded_nal::Ipv4Addr::from(std_ip4!(sockaddr))),
                sockaddr.port(),
            ),
        ))
    }
}
