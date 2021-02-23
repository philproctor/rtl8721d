pub struct UdpSocket {
    fd: i32,
}

impl UdpSocket {
    pub fn new() -> Self {
        let fd;
        unsafe {
            fd = c::lwip_socket(
                c::AF_INET as i32,
                c::SOCK_DGRAM as i32,
                c::IPPROTO_UDP as i32,
            );
        }
        UdpSocket { fd }
    }
}
