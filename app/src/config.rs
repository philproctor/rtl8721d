use crate::prelude::*;
use core::cell::UnsafeCell;
use heapless::String;

pub static CONFIG: ConfigContainer = ConfigContainer::new();

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum WifiMode {
    Open = 0,
    WPA2 = 1,
    UNSET = 255,
}

#[repr(C)]
#[derive(Debug, Clone)]
struct Configuration {
    wifi_mode: WifiMode,
    ssid: String<consts::U33>,
    password: String<consts::U64>,
}

impl Configuration {
    pub const fn new() -> Self {
        Self {
            wifi_mode: WifiMode::Open,
            ssid: String(heapless::i::String::new()),
            password: String(heapless::i::String::new()),
        }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}

pub struct ConfigContainer(UnsafeCell<Configuration>);

unsafe impl Send for ConfigContainer {}
unsafe impl Sync for ConfigContainer {}

impl ConfigContainer {
    const SIZE: usize = core::mem::size_of::<Configuration>();

    const fn new() -> Self {
        ConfigContainer(UnsafeCell::new(Configuration::new()))
    }

    fn inner(&self) -> &mut Configuration {
        unsafe { &mut *self.0.get() }
    }

    pub fn save(&self) -> core::result::Result<(), crate::error::SystemError> {
        let cpy = self.inner().clone();
        let bytes: [u8; Self::SIZE] = unsafe { core::mem::transmute(cpy) };
        STORAGE.write(0, &bytes)
    }

    pub fn load(&self) -> core::result::Result<(), crate::error::SystemError> {
        let bytes = STORAGE.read(0, Self::SIZE as u32)?;
        let mut sb = [0u8; Self::SIZE];
        sb.copy_from_slice(&bytes[..Self::SIZE]);
        let cfg: Self = unsafe { core::mem::transmute(sb) };
        if cfg.inner().is_valid() {
            self.set_wifi_mode(cfg.get_wifi_mode());
            self.set_ssid(cfg.get_ssid().clone());
            self.set_password(cfg.get_password().clone());
            Ok(())
        } else {
            Err(SystemError::Invalid("Configuration self check failed"))
        }
    }

    pub fn get_wifi_mode(&self) -> WifiMode {
        self.inner().wifi_mode
    }

    pub fn get_ssid(&self) -> &String<consts::U33> {
        &self.inner().ssid
    }

    pub fn get_password(&self) -> &String<consts::U64> {
        &self.inner().password
    }

    pub fn set_wifi_mode(&self, val: WifiMode) {
        self.inner().wifi_mode = val
    }

    pub fn set_ssid(&self, val: String<consts::U33>) {
        self.inner().ssid = val
    }

    pub fn set_password(&self, val: String<consts::U64>) {
        self.inner().password = val
    }
}
