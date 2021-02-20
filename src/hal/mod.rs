mod net;
mod rtos;
mod serial;
mod sys;
mod wifi;

pub use net::{DHCPAction, Network};
pub use rtos::{RtosAllocator, RTOS};
pub use serial::SERIAL1;
pub use sys::System;
pub use wifi::{Wifi, WifiEncryption, WifiInterface, WifiResultCode, WifiSecurityMode};

pub unsafe fn init() {
    serial::init();
}
