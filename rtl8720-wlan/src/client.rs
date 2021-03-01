use crate::*;
use rtl8720_sys as sys;

// It appears that the APIs maintain references after usage in functions, so using these static memory
// locations seems to be the safest fix as these variables are always needed
static mut PASSWORD: [u8; 64] = [0u8; 64];
static mut WIFI: sys::c::rtw_network_info_t = sys::c::rtw_network_info_t {
    ssid: sys::c::rtw_ssid {
        val: [0u8; 33],
        len: 0,
    },
    bssid: sys::c::rtw_mac { octet: [0u8; 6] },
    key_id: -1,
    password: core::ptr::null_mut(),
    password_len: 0,
    security_type: 0,
};

pub struct WifiClient;

impl WifiExtensions<{ WifiInterface::Station as i32 }, { WifiMode::Station as u32 }>
    for WifiClient
{
    fn mode_init() -> Result<()> {
        Self::autoreconnect(true)?;
        Ok(())
    }
}

impl WifiClient {
    pub fn connect_wpa2(ssid: &str, pw: &str) -> Result<()> {
        unsafe {
            WIFI.ssid.val = core::mem::zeroed();
            let ssid_bytes = ssid.as_bytes();
            let pw_bytes = pw.as_bytes();
            WIFI.ssid.val[..ssid_bytes.len()].copy_from_slice(ssid.as_bytes());
            WIFI.ssid.len = ssid.len() as u8;
            PASSWORD = core::mem::zeroed();
            PASSWORD[..pw_bytes.len()].copy_from_slice(pw_bytes);
            WIFI.password = PASSWORD.as_mut_ptr();
            WIFI.password_len = pw.len() as i32;
            WIFI.security_type = WifiSecurityMode::Wpa2AesPsk as u32;
            WifiResultCode::i32_to_result(
                sys::c::wifi_connect(
                    &mut *WIFI.ssid.val.as_mut_ptr(),
                    WIFI.security_type,
                    &mut *PASSWORD.as_mut_ptr(),
                    WIFI.ssid.len as i32,
                    WIFI.password_len,
                    -1,
                    &mut *core::ptr::null_mut(),
                ),
                (),
            )
        }
    }

    pub fn disconnect() -> Result<()> {
        WifiResultCode::i32_to_result(unsafe { sys::c::wifi_disconnect() }, ())
    }

    pub fn autoreconnect(on: bool) -> Result<()> {
        WifiResultCode::i32_to_result(unsafe { sys::c::wifi_set_autoreconnect(on as u8) }, ())
    }
}
