use crate::prelude::*;

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiMode {
    None = c::RTW_MODE_NONE as u32,
    Station = c::RTW_MODE_STA as u32,
    AccessPoint = c::RTW_MODE_AP as u32,
    StationAndAccessPoint = c::RTW_MODE_STA_AP as u32,
    Promiscuous = c::RTW_MODE_PROMISC as u32,
    PeerToPeer = c::RTW_MODE_P2P as u32,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiInterface {
    Station = c::RTW_STA_INTERFACE as u32,
    AccessPoint = c::RTW_AP_INTERFACE as u32,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiEncryption {
    Unknown = c::RTW_ENCRYPTION_UNKNOWN as u32,
    Open = c::RTW_ENCRYPTION_OPEN as u32,
    Wep40 = c::RTW_ENCRYPTION_WEP40 as u32,
    WpaTkip = c::RTW_ENCRYPTION_WPA_TKIP as u32,
    WpaAes = c::RTW_ENCRYPTION_WPA_AES as u32,
    Wpa2Tkip = c::RTW_ENCRYPTION_WPA2_TKIP as u32,
    Wpa2Aes = c::RTW_ENCRYPTION_WPA2_AES as u32,
    WPA2Mixed = c::RTW_ENCRYPTION_WPA2_MIXED as u32,
    Wep104 = c::RTW_ENCRYPTION_WEP104 as u32,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiSecurityMode {
    Open = c::RTW_SECURITY_OPEN as u32,
    WepPSK = c::RTW_SECURITY_WEP_PSK as u32,
    WepShared = c::RTW_SECURITY_WEP_SHARED as u32,
    WpaTkipPsk = c::RTW_SECURITY_WPA_TKIP_PSK as u32,
    WpaAesPsk = c::RTW_SECURITY_WPA_AES_PSK as u32,
    Wpa2AesPsk = c::RTW_SECURITY_WPA2_AES_PSK as u32,
    Wpa2TkipPsk = c::RTW_SECURITY_WPA2_TKIP_PSK as u32,
    Wpa2MixedPsk = c::RTW_SECURITY_WPA2_MIXED_PSK as u32,
    WpaWpa2Mixed = c::RTW_SECURITY_WPA_WPA2_MIXED as u32,
    Wpa2AesCmac = c::RTW_SECURITY_WPA2_AES_CMAC as u32,
    Wpa2Enterprise = c::RTW_SECURITY_WPA2_ENTERPRISE as u32,
    WpaWpa2Enterprise = c::RTW_SECURITY_WPA_WPA2_ENTERPRISE as u32,
    WpsOpen = c::RTW_SECURITY_WPS_OPEN as u32,
    WpsSecure = c::RTW_SECURITY_WPS_SECURE as u32,
    Wpa3AesPsk = c::RTW_SECURITY_WPA3_AES_PSK as u32,
    Unknown = c::RTW_SECURITY_UNKNOWN as u32,
    Force32Bit = c::RTW_SECURITY_FORCE_32_BIT as u32,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiResultCode {
    Success = c::RTW_SUCCESS as i32,
    Pending = c::RTW_PENDING as i32,
    Timeout = c::RTW_TIMEOUT as i32,
    PartialResult = c::RTW_PARTIAL_RESULTS as i32,
    InvalidKey = c::RTW_INVALID_KEY as i32,
    DoesNotExist = c::RTW_DOES_NOT_EXIST as i32,
    NotAuthenticated = c::RTW_NOT_AUTHENTICATED as i32,
    NotKeyed = c::RTW_NOT_KEYED as i32,
    IoctlFail = c::RTW_IOCTL_FAIL as i32,
    BufferTemporarilyUnavailable = c::RTW_BUFFER_UNAVAILABLE_TEMPORARY as i32,
    BufferPermanentlyUnavailable = c::RTW_BUFFER_UNAVAILABLE_PERMANENT as i32,
    WpsPbcOverlap = c::RTW_WPS_PBC_OVERLAP as i32,
    ConnectionLost = c::RTW_CONNECTION_LOST as i32,
    Error = c::RTW_ERROR as i32,
    BadArgument = c::RTW_BADARG as i32,
    BadOption = c::RTW_BADOPTION as i32,
    NotUp = c::RTW_NOTUP as i32,
    NotDown = c::RTW_NOTDOWN as i32,
    NotAP = c::RTW_NOTAP as i32,
    NotSta = c::RTW_NOTSTA as i32,
    BadKeyIndex = c::RTW_BADKEYIDX as i32,
    RadioOff = c::RTW_RADIOOFF as i32,
    NotBandLocked = c::RTW_NOTBANDLOCKED as i32,
    NoClock = c::RTW_NOCLK as i32,
    BadRateSet = c::RTW_BADRATESET as i32,
    BadBand = c::RTW_BADBAND as i32,
    BufferTooShort = c::RTW_BUFTOOSHORT as i32,
    BufferTooLong = c::RTW_BUFTOOLONG as i32,
    Busy = c::RTW_BUSY as i32,
    NotAssociated = c::RTW_NOTASSOCIATED as i32,
    BadSSIDLength = c::RTW_BADSSIDLEN as i32,
    OutOfRangeChannel = c::RTW_OUTOFRANGECHAN as i32,
    BadChannel = c::RTW_BADCHAN as i32,
    BadAddress = c::RTW_BADADDR as i32,
    NoResource = c::RTW_NORESOURCE as i32,
    Unsupported = c::RTW_UNSUPPORTED as i32,
    BadLength = c::RTW_BADLEN as i32,
    NotReady = c::RTW_NOTREADY as i32,
    ErrorPermanent = c::RTW_EPERM as i32,
    NoMemory = c::RTW_NOMEM as i32,
    Associated = c::RTW_ASSOCIATED as i32,
    Range = c::RTW_RANGE as i32,
    NotFound = c::RTW_NOTFOUND as i32,
    WmeNotEnabled = c::RTW_WME_NOT_ENABLED as i32,
    TSpecNotFound = c::RTW_TSPEC_NOTFOUND as i32,
    AcmNotSupported = c::RTW_ACM_NOTSUPPORTED as i32,
    NotWmeAssociated = c::RTW_NOT_WME_ASSOCIATION as i32,
    SdioError = c::RTW_SDIO_ERROR as i32,
    WlanDown = c::RTW_WLAN_DOWN as i32,
    BadVersion = c::RTW_BAD_VERSION as i32,
    TxFail = c::RTW_TXFAIL as i32,
    RxFail = c::RTW_RXFAIL as i32,
    NoDevice = c::RTW_NODEVICE as i32,
    Unfinished = c::RTW_UNFINISHED as i32,
    Nonresident = c::RTW_NONRESIDENT as i32,
    Disabled = c::RTW_DISABLED as i32,
}

impl WifiResultCode {
    fn i32_to_result(code: i32) -> Result<()> {
        if code != 0 {
            Err(SystemError::Wifi(
                WifiResultCode::from_i32(code).unwrap_or(WifiResultCode::Error),
            ))
        } else {
            Ok(())
        }
    }
}

fn wifi_result(code: i32) -> Result<()> {
    WifiResultCode::i32_to_result(code)
}

// It appears that the APIs maintain references after usage in functions, so using these static memory
// locations seems to be the safest fix as these variables are always needed
static mut PASSWORD: [u8; 64] = [0u8; 64];
static mut WIFI: c::rtw_network_info_t = c::rtw_network_info_t {
    ssid: c::rtw_ssid {
        val: [0u8; 33],
        len: 0,
    },
    bssid: c::rtw_mac { octet: [0u8; 6] },
    key_id: -1,
    password: core::ptr::null_mut(),
    password_len: 0,
    security_type: 0,
};

pub struct Wifi {}

impl Wifi {
    pub fn init() -> Result<()> {
        Self::on(WifiMode::Station)?;
        Self::autoreconnect(true)?;
        Ok(())
    }

    pub fn on(mode: WifiMode) -> Result<()> {
        wifi_result(unsafe { c::wifi_on(mode as u32) })
    }

    pub fn off() -> Result<()> {
        wifi_result(unsafe { c::wifi_off() })
    }

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
            // RTOS::spawn(connect_wpa2_fn, "wifi_connect");
            wifi_result(c::wifi_connect(
                &mut *WIFI.ssid.val.as_mut_ptr(),
                WIFI.security_type,
                &mut *PASSWORD.as_mut_ptr(),
                WIFI.ssid.len as i32,
                WIFI.password_len,
                -1,
                &mut *core::ptr::null_mut(),
            ))
        }
        // Ok(())
    }

    pub fn disconnect() -> Result<()> {
        wifi_result(unsafe { c::wifi_disconnect() })
    }

    pub fn get_mac_address() -> Result<String> {
        unsafe {
            let mut mac = vec![0u8;18];
            wifi_result(c::wifi_get_mac_address(mac.as_mut_ptr()))?;
            let mac: String = String::from_utf8(mac)?;
            Ok(mac)
        }
    }

    pub fn is_connected() -> bool {
        unsafe { c::wifi_is_connected_to_ap() == 1 }
    }

    pub fn autoreconnect(on: bool) -> Result<()> {
        wifi_result(unsafe { c::wifi_set_autoreconnect(on as u8) })
    }

    pub fn wifi_scan() {
        unsafe { c::wifi_scan_networks(Some(scan_handler), &mut *core::ptr::null_mut()) };
    }

    pub fn get_last_error() -> String {
        match unsafe { c::wifi_get_last_error() } {
            i if i == (c::RTW_NO_ERROR as i32) => "None".into(),
            i if i == (c::RTW_NONE_NETWORK as i32) => "AP not found".into(),
            i if i == (c::RTW_CONNECT_FAIL as i32) => "Association failed".into(),
            i if i == (c::RTW_WRONG_PASSWORD as i32) => "Wrong password".into(),
            i if i == (c::RTW_4WAY_HANDSHAKE_TIMEOUT as i32) => "Handshake failed".into(),
            i if i == (c::RTW_DHCP_FAIL as i32) => "DHCP failed".into(),
            i if i == (c::RTW_UNKNOWN as i32) => "Unknown".into(),
            i => format!("Unexpected code {}", i),
        }
    }
}

// wifi_reg_event_handler
// wifi_unreg_event_handler
// wifi_indication

unsafe extern "C" fn scan_handler(r: *mut c::rtw_scan_handler_result) -> i32 {
    let ap_details = &(*r).ap_details;
    let ssid_len = ap_details.SSID.len as usize;
    let ssid_sl = ap_details.SSID.val[..ssid_len].to_vec();
    let ssid = String::from_utf8(ssid_sl).unwrap_or_else(|_| "(unprintable)".into());
    let enc = WifiSecurityMode::from_u32(ap_details.security).unwrap_or(WifiSecurityMode::Unknown);
    let rssi = ap_details.signal_strength;
    SERIAL1.tx_string(&format!(
        "Found SSID: '{}' => mode {:?} => rssi {}",
        ssid, enc, rssi
    ));
    1
}
