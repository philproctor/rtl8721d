use crate::*;
use ::heapless;
use rtl8720_sys::{c, ctypes::*};

pub type LwipResult<T> = embedded_nal::nb::Result<T, LwipError>;
pub type AcceptChannel = heapless::mpmc::Q8<TcpPcb>;
pub type PbufChannel = heapless::spsc::Queue<*mut c::pbuf, heapless::consts::U32>;

static mut LISTENERS: heapless::LinearMap<u16, AcceptChannel, heapless::consts::U8> =
    heapless::LinearMap(heapless::i::LinearMap::new());

fn register_listener(port: u16, channel: AcceptChannel) {
    if let Ok(_) = unsafe { LISTENERS.insert(port, channel) } {}
}

fn get_listener_channel(port: u16) -> &'static mut AcceptChannel {
    unsafe { LISTENERS.get_mut(&port) }.unwrap()
}

macro_rules! unroll {
    ($zelf:expr , $unroll:ident ) => {
        match $zelf {
            Self::New { $unroll, .. }
            | Self::Bound { $unroll, .. }
            | Self::ClientConnected { $unroll, .. }
            | Self::Listening { $unroll, .. }
            | Self::Connected { $unroll, .. }
            | Self::Connecting { $unroll, .. } => Some($unroll),
            Self::Closed { .. } => None,
        }
    };
}

macro_rules! check {
    ($code:expr) => {
        match LwipErrCode::from_i32($code as i32) {
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

macro_rules! result_code {
    ($res:expr, $code:expr) => {
        match LwipErrCode::from_i32($code as i32) {
            Some(LwipErrCode::Ok) => Ok($res),
            None => Err(nb::Error::Other(LwipError::Unexpected)),
            Some(e) => Err(nb::Error::Other(LwipError::Code(e))),
        }
    };
}

macro_rules! with_state {
    ($zelf:expr , $( $from:pat )|+ => { $( $to:tt )* }) => {
        match $zelf {
            $( $from )|* => { $( $to )* },
            _ => invalid_state!("Invalid socket state for this operation..."),
        }
    };
}

macro_rules! transition_to {
    ($zelf:expr , $from:pat => { $( $to:tt )* }) => {
        match $zelf {
            $from => *$zelf = { $( $to )* },
            _ => invalid_state!("Invalid socket state for this operation..."),
        }
    };
}

macro_rules! invalid_state {
    ($lit:literal) => {
        return Err(embedded_nal::nb::Error::Other(LwipError::InvalidState(
            $lit,
        )));
    };
}

// #[derive(Debug)]
pub enum LwipSocket {
    New {
        inner: TcpPcb,
    },
    Bound {
        inner: TcpPcb,
    },
    Connecting {
        inner: TcpPcb,
    },
    Connected {
        inner: TcpPcb,
        channel: PbufChannel,
        buf: Pbuf,
    },
    ClientConnected {
        inner: TcpPcb,
        channel: PbufChannel,
        buf: Pbuf,
    },
    Listening {
        inner: TcpPcb,
        channel: &'static mut AcceptChannel,
    },
    Closed {},
}

impl LwipSocket {
    pub fn new() -> Self {
        lwip_debug!("new socket");
        Self::New {
            inner: TcpPcb::new(),
        }
    }

    unsafe fn new_client(mut inner: TcpPcb) -> Self {
        lwip_debug!("new client");
        let channel = PbufChannel::new();
        // inner.set_callback_arg(TcpPcb::ptr_to_usize(&mut channel));
        inner.set_recv_fn(Self::recv_fn);
        let res = Self::ClientConnected {
            buf: Pbuf::empty(),
            inner,
            channel,
        };
        res
    }

    pub fn send(&mut self, data: &[u8]) -> LwipResult<usize> {
        match self {
            Self::Connected { inner, .. } | Self::ClientConnected { inner, .. } => {
                let res = inner.write(data);
                result_code!(data.len(), res)
            }
            _ => invalid_state!("Can only send data on connected sockets!"),
        }
    }

    pub fn recv(&mut self, outbuf: &mut [u8]) -> LwipResult<u16> {
        with_state!(self,
            Self::Connected{ channel, buf, .. }
            | Self::ClientConnected{ channel, buf, .. } => {
                while channel.len() > 0 {
                    let pref = channel.dequeue().unwrap();
                    buf.link(pref);
                    unsafe { c::pbuf_free(pref) };
                }
                Ok(buf.read_to(outbuf))
            }
        )
    }

    pub fn get_state(&self) -> TCPState {
        match self {
            Self::New { .. } => TCPState::Initialized,
            Self::Bound { .. } => TCPState::Bound,
            Self::ClientConnected { .. } => TCPState::Connected,
            Self::Connecting { .. } => TCPState::Connecting,
            Self::Connected { .. } => TCPState::Connected,
            Self::Listening { .. } => TCPState::Listening,
            Self::Closed { .. } => TCPState::Closed,
        }
    }

    pub fn connect(&mut self, ipaddr: c::ip_addr_t, port: c::u16_t) -> LwipResult<()> {
        lwip_debug!("connect start");
        transition_to!(self, Self::New { inner, .. } => {
            check!(inner.connect(ipaddr, port, Self::connect_fn));
            Self::Connecting { inner: inner.take() }
        });
        lwip_debug!("connect end");
        Ok(())
    }

    pub fn bind(&mut self, port: c::u16_t) -> LwipResult<()> {
        lwip_debug!("bind start");
        transition_to!(self, Self::New { inner, .. } => {
            check!(inner.bind(port));
            Self::Bound { inner: inner.take() }
        });
        lwip_debug!("bind end");
        Ok(())
    }

    pub fn listen(&mut self) -> LwipResult<()> {
        lwip_debug!("listen start");
        transition_to!(self, Self::Bound { inner, .. } => {
            lwip_debug!("inner");
            check!(inner.listen());
            let channel = AcceptChannel::new();
            let p = unsafe { (*inner.listen_ptr()).local_port };
            register_listener(p, channel);
            // inner.set_callback_arg(TcpPcb::ptr_to_usize(&mut channel));
            inner.set_callback_arg(p as usize);
            inner.set_accept_fn(Self::accept_fn);
            Self::Listening { inner: inner.take(), channel: get_listener_channel(p) }
        });
        lwip_debug!("listen end");
        Ok(())
    }

    pub fn accept(&mut self) -> LwipResult<Option<Self>> {
        with_state!(self, Self::Listening{ channel, .. } => {
            // if channel.len() > 0 {
            //     lwip_debug!(".");
            // }
            match channel.dequeue() {
                Some(new_pcb) => {
                    return Ok(Some(unsafe { Self::new_client(new_pcb) }))
                }
                None => return Ok(None),
            };
        });
    }

    pub fn close(&mut self) -> LwipResult<()> {
        lwip_debug!("Closing socket...");
        let inner = unroll!(self, inner);
        if let Some(inner) = inner {
            let res = inner.close();
            *self = Self::Closed {};
            result_code!((), res)
        } else {
            Ok(())
        }
    }

    pub fn remote_ip(&self) -> IpAddr {
        let pcb = unroll!(self, inner).unwrap();
        c_ip4_to_native!((*pcb.listen_ptr()).remote_ip.addr)
    }

    pub fn remote_port(&self) -> u16 {
        let pcb = unroll!(self, inner).unwrap();
        unsafe { *pcb.listen_ptr() }.local_port
    }

    pub fn remote_sockaddr(&self) -> SocketAddr {
        SocketAddr::new(self.remote_ip(), self.remote_port())
    }

    extern "C" fn accept_fn(port: *mut c_void, newcon: *mut c::tcp_pcb, _: i8) -> i8 {
        let conn = unsafe { TcpPcb::new_from_ptr(newcon) };
        let port = TcpPcb::ptr_to_usize(port);
        let channel = get_listener_channel(port as u16);
        match channel.enqueue(conn) {
            Ok(()) => {
                lwip_debug!("Accept complete");
            }
            Err(e) => {
                lwip_debug!("Accept error: {:?}", e);
            }
        };
        lwip_debug!("accept_fn end");
        0
    }

    extern "C" fn recv_fn(
        channel: *mut c_void,
        _: *mut c::tcp_pcb,
        buf: *mut c::pbuf,
        _: i8,
    ) -> i8 {
        lwip_debug!("recv_fn start");
        let channel: &mut PbufChannel = unsafe { core::mem::transmute(channel) };
        match channel.enqueue(buf) {
            Ok(()) => {
                unsafe { c::pbuf_ref(buf) };
                lwip_debug!("Pbuf queued");
            }
            Err(e) => {
                lwip_debug!("Pbuf queue failure: {:?}", e);
            }
        };
        lwip_debug!("recv_fn end");
        0
    }

    extern "C" fn connect_fn(sself: *mut c_void, _: *mut c::tcp_pcb, _: i8) -> i8 {
        // TODO: Possible this is broke
        lwip_debug!("connect_fn start");
        let sself = unsafe { &mut *sself.cast::<LwipSocket>() };
        let inner = unroll!(sself, inner).unwrap().take();
        *sself = LwipSocket::Connected {
            inner,
            channel: PbufChannel::new(),
            buf: unsafe { Pbuf::empty() },
        };
        lwip_debug!("connect_fn end");
        0
    }
}
