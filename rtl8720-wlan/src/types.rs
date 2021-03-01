use num::FromPrimitive;
use rtl8720_sys as sys;

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiMode {
    None = sys::c::RTW_MODE_NONE as u32,
    Station = sys::c::RTW_MODE_STA as u32,
    AccessPoint = sys::c::RTW_MODE_AP as u32,
    StationAndAccessPoint = sys::c::RTW_MODE_STA_AP as u32,
    Promiscuous = sys::c::RTW_MODE_PROMISC as u32,
    PeerToPeer = sys::c::RTW_MODE_P2P as u32,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiInterface {
    Station = sys::c::RTW_STA_INTERFACE as u32,
    AccessPoint = sys::c::RTW_AP_INTERFACE as u32,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiEncryption {
    Unknown = sys::c::RTW_ENCRYPTION_UNKNOWN as u32,
    Open = sys::c::RTW_ENCRYPTION_OPEN as u32,
    Wep40 = sys::c::RTW_ENCRYPTION_WEP40 as u32,
    WpaTkip = sys::c::RTW_ENCRYPTION_WPA_TKIP as u32,
    WpaAes = sys::c::RTW_ENCRYPTION_WPA_AES as u32,
    Wpa2Tkip = sys::c::RTW_ENCRYPTION_WPA2_TKIP as u32,
    Wpa2Aes = sys::c::RTW_ENCRYPTION_WPA2_AES as u32,
    WPA2Mixed = sys::c::RTW_ENCRYPTION_WPA2_MIXED as u32,
    Wep104 = sys::c::RTW_ENCRYPTION_WEP104 as u32,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiSecurityMode {
    Open = sys::c::RTW_SECURITY_OPEN as u32,
    WepPSK = sys::c::RTW_SECURITY_WEP_PSK as u32,
    WepShared = sys::c::RTW_SECURITY_WEP_SHARED as u32,
    WpaTkipPsk = sys::c::RTW_SECURITY_WPA_TKIP_PSK as u32,
    WpaAesPsk = sys::c::RTW_SECURITY_WPA_AES_PSK as u32,
    Wpa2AesPsk = sys::c::RTW_SECURITY_WPA2_AES_PSK as u32,
    Wpa2TkipPsk = sys::c::RTW_SECURITY_WPA2_TKIP_PSK as u32,
    Wpa2MixedPsk = sys::c::RTW_SECURITY_WPA2_MIXED_PSK as u32,
    WpaWpa2Mixed = sys::c::RTW_SECURITY_WPA_WPA2_MIXED as u32,
    Wpa2AesCmac = sys::c::RTW_SECURITY_WPA2_AES_CMAC as u32,
    Wpa2Enterprise = sys::c::RTW_SECURITY_WPA2_ENTERPRISE as u32,
    WpaWpa2Enterprise = sys::c::RTW_SECURITY_WPA_WPA2_ENTERPRISE as u32,
    WpsOpen = sys::c::RTW_SECURITY_WPS_OPEN as u32,
    WpsSecure = sys::c::RTW_SECURITY_WPS_SECURE as u32,
    Wpa3AesPsk = sys::c::RTW_SECURITY_WPA3_AES_PSK as u32,
    Unknown = sys::c::RTW_SECURITY_UNKNOWN as u32,
    Force32Bit = sys::c::RTW_SECURITY_FORCE_32_BIT as u32,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WifiResultCode {
    Success = sys::c::RTW_SUCCESS as i32,
    Pending = sys::c::RTW_PENDING as i32,
    Timeout = sys::c::RTW_TIMEOUT as i32,
    PartialResult = sys::c::RTW_PARTIAL_RESULTS as i32,
    InvalidKey = sys::c::RTW_INVALID_KEY as i32,
    DoesNotExist = sys::c::RTW_DOES_NOT_EXIST as i32,
    NotAuthenticated = sys::c::RTW_NOT_AUTHENTICATED as i32,
    NotKeyed = sys::c::RTW_NOT_KEYED as i32,
    IoctlFail = sys::c::RTW_IOCTL_FAIL as i32,
    BufferTemporarilyUnavailable = sys::c::RTW_BUFFER_UNAVAILABLE_TEMPORARY as i32,
    BufferPermanentlyUnavailable = sys::c::RTW_BUFFER_UNAVAILABLE_PERMANENT as i32,
    WpsPbcOverlap = sys::c::RTW_WPS_PBC_OVERLAP as i32,
    ConnectionLost = sys::c::RTW_CONNECTION_LOST as i32,
    Error = sys::c::RTW_ERROR as i32,
    BadArgument = sys::c::RTW_BADARG as i32,
    BadOption = sys::c::RTW_BADOPTION as i32,
    NotUp = sys::c::RTW_NOTUP as i32,
    NotDown = sys::c::RTW_NOTDOWN as i32,
    NotAP = sys::c::RTW_NOTAP as i32,
    NotSta = sys::c::RTW_NOTSTA as i32,
    BadKeyIndex = sys::c::RTW_BADKEYIDX as i32,
    RadioOff = sys::c::RTW_RADIOOFF as i32,
    NotBandLocked = sys::c::RTW_NOTBANDLOCKED as i32,
    NoClock = sys::c::RTW_NOCLK as i32,
    BadRateSet = sys::c::RTW_BADRATESET as i32,
    BadBand = sys::c::RTW_BADBAND as i32,
    BufferTooShort = sys::c::RTW_BUFTOOSHORT as i32,
    BufferTooLong = sys::c::RTW_BUFTOOLONG as i32,
    Busy = sys::c::RTW_BUSY as i32,
    NotAssociated = sys::c::RTW_NOTASSOCIATED as i32,
    BadSSIDLength = sys::c::RTW_BADSSIDLEN as i32,
    OutOfRangeChannel = sys::c::RTW_OUTOFRANGECHAN as i32,
    BadChannel = sys::c::RTW_BADCHAN as i32,
    BadAddress = sys::c::RTW_BADADDR as i32,
    NoResource = sys::c::RTW_NORESOURCE as i32,
    Unsupported = sys::c::RTW_UNSUPPORTED as i32,
    BadLength = sys::c::RTW_BADLEN as i32,
    NotReady = sys::c::RTW_NOTREADY as i32,
    ErrorPermanent = sys::c::RTW_EPERM as i32,
    NoMemory = sys::c::RTW_NOMEM as i32,
    Associated = sys::c::RTW_ASSOCIATED as i32,
    Range = sys::c::RTW_RANGE as i32,
    NotFound = sys::c::RTW_NOTFOUND as i32,
    WmeNotEnabled = sys::c::RTW_WME_NOT_ENABLED as i32,
    TSpecNotFound = sys::c::RTW_TSPEC_NOTFOUND as i32,
    AcmNotSupported = sys::c::RTW_ACM_NOTSUPPORTED as i32,
    NotWmeAssociated = sys::c::RTW_NOT_WME_ASSOCIATION as i32,
    SdioError = sys::c::RTW_SDIO_ERROR as i32,
    WlanDown = sys::c::RTW_WLAN_DOWN as i32,
    BadVersion = sys::c::RTW_BAD_VERSION as i32,
    TxFail = sys::c::RTW_TXFAIL as i32,
    RxFail = sys::c::RTW_RXFAIL as i32,
    NoDevice = sys::c::RTW_NODEVICE as i32,
    Unfinished = sys::c::RTW_UNFINISHED as i32,
    Nonresident = sys::c::RTW_NONRESIDENT as i32,
    Disabled = sys::c::RTW_DISABLED as i32,
}

pub(crate) type Result<T> = core::result::Result<T, WifiResultCode>;

impl WifiResultCode {
    pub fn i32_to_result<T>(code: i32, ok: T) -> Result<T> {
        if code != 0 {
            Err(WifiResultCode::from_i32(code).unwrap_or(WifiResultCode::Error))
        } else {
            Ok(ok)
        }
    }
}
