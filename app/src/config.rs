use crate::prelude::*;
use core::cell::UnsafeCell;
use heapless::String;

pub static CONFIG: Configuration = Configuration::new();

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum WifiMode {
    Open = 0,
    WPA2 = 1,
    UNSET = 255,
}

#[repr(C)]
#[derive(Debug)]
pub struct Configuration {
    wifi_mode: UnsafeCell<WifiMode>,
    ssid: UnsafeCell<String<consts::U33>>,
    password: UnsafeCell<String<consts::U64>>,
}

unsafe impl Send for Configuration {}
unsafe impl Sync for Configuration {}

const CONFIG_SIZE: usize = core::mem::size_of::<Configuration>();

impl Configuration {
    pub const fn new() -> Self {
        Self {
            wifi_mode: UnsafeCell::new(WifiMode::Open),
            ssid: UnsafeCell::new(String(heapless::i::String::new())),
            password: UnsafeCell::new(String(heapless::i::String::new())),
        }
    }

    pub fn save(&self) -> core::result::Result<(), crate::error::SystemError> {
        let cpy = unsafe {
            Self {
                wifi_mode: UnsafeCell::new(*self.wifi_mode.get()),
                ssid: UnsafeCell::new((*self.ssid.get()).clone()),
                password: UnsafeCell::new((*self.password.get()).clone()),
            }
        };
        let bytes: [u8; CONFIG_SIZE] = unsafe { core::mem::transmute(cpy) };
        STORAGE.write(0, &bytes)
    }

    pub fn load(&self) -> core::result::Result<(), crate::error::SystemError> {
        let bytes = STORAGE.read(0, CONFIG_SIZE as u32)?;
        let mut sb = [0u8; CONFIG_SIZE];
        sb.copy_from_slice(&bytes[..CONFIG_SIZE]);
        let cfg: Self = unsafe { core::mem::transmute(sb) };
        self.set_wifi_mode(cfg.get_wifi_mode());
        self.set_ssid(cfg.get_ssid().clone());
        self.set_password(cfg.get_password().clone());
        Ok(())
    }

    pub fn get_wifi_mode(&self) -> WifiMode {
        unsafe { *self.wifi_mode.get() }
    }

    pub fn get_ssid(&self) -> &String<consts::U33> {
        unsafe { &*self.ssid.get() }
    }

    pub fn get_password(&self) -> &String<consts::U64> {
        unsafe { &*self.password.get() }
    }

    pub fn set_wifi_mode(&self, val: WifiMode) {
        *unsafe { &mut *self.wifi_mode.get() } = val
    }

    pub fn set_ssid(&self, val: String<consts::U33>) {
        *unsafe { &mut *self.ssid.get() } = unsafe { core::mem::zeroed() };
        *unsafe { &mut *self.ssid.get() } = val;
    }

    pub fn set_password(&self, val: String<consts::U64>) {
        *unsafe { &mut *self.password.get() } = unsafe { core::mem::zeroed() };
        *unsafe { &mut *self.password.get() } = val
    }
}
