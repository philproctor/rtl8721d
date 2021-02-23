pub use crate::ffi::c;
use crate::prelude::*;

pub enum Host {
    IPAddress([u8; 4]),
    UnresolvedHost(String),
    ResolvedHost(String, [u8; 4]),
}

impl Host {
    pub fn ip_string(&self) -> String {
        match self {
            Self::ResolvedHost(_, ip) | Self::IPAddress(ip) => {
                format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
            }
            _ => "(unknown)".into(),
        }
    }

    pub fn ip_u32(&self) -> u32 {
        match self {
            Self::ResolvedHost(_, ip) | Self::IPAddress(ip) => u32::from_ne_bytes(ip.clone()),
            _ => 0,
        }
    }

    pub fn ip4_addr(&self) -> c::ip4_addr {
        c::ip4_addr {
            addr: self.ip_u32(),
        }
    }

    pub fn u8_to_ip4(b: [u8; 4]) -> c::ip4_addr {
        c::ip4_addr {
            addr: u32::from_ne_bytes(b),
        }
    }

    pub fn host_str(&self) -> &str {
        match self {
            Self::ResolvedHost(host, _) | Self::UnresolvedHost(host) => host,
            _ => "(unknown)",
        }
    }
}

impl From<[u8; 4]> for Host {
    fn from(ip: [u8; 4]) -> Self {
        Self::IPAddress(ip)
    }
}
