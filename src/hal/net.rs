use crate::ffi::*;
// use crate::prelude::*;

pub struct Network;

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum DHCPAction {
    Start = c::DHCP_State_TypeDef_DHCP_START as u32,
    WaitAddress = c::DHCP_State_TypeDef_DHCP_WAIT_ADDRESS as u32,
    AddressAssigned = c::DHCP_State_TypeDef_DHCP_ADDRESS_ASSIGNED as u32,
    ReleaseIP = c::DHCP_State_TypeDef_DHCP_RELEASE_IP as u32,
    Stop = c::DHCP_State_TypeDef_DHCP_STOP as u32,
    Timeout = c::DHCP_State_TypeDef_DHCP_TIMEOUT as u32,
}

impl Network {
    pub fn init() {
        unsafe { c::LwIP_Init() };
    }

    pub fn dhcp(action: DHCPAction) -> u8 {
        // As far as I can tell, there's no reason idx would ever not be 0
        unsafe { c::LwIP_DHCP(0, action as u8) }
    }

    pub fn get_ip() -> [u8;4] {
        let mut ip = [0u8;4];
        unsafe {
            let ip_ptr = c::LwIP_GetIP_Idx(0);
            ip.copy_from_slice(core::slice::from_raw_parts(ip_ptr, 4));
        }
        ip
    }

    // LwIP_GetGW_Idx
    // LwIP_GetMASK_Idx
    // LwIP_GetDNS
    // lwip_socket()
}
