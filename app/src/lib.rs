#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(lang_items)]
#![feature(try_trait)]
#![feature(associated_type_bounds)]
#![allow(unused_attributes)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_harness::runner)]
#![feature(const_raw_ptr_deref)]
#![feature(const_fn_fn_ptr_basics)]
// #![reexport_test_harness_main = "test_main"]

extern crate alloc;

// #[macro_use]
// extern crate num_derive;

#[macro_use]
mod log;

pub mod device;

pub mod config;
pub mod prelude;

mod error;
mod executor;
mod hal;
mod tasks;
mod util;

use rtl8720_sys as ffi;

use prelude::*;
use tasks::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        SERIAL1.init();
        STORAGE.init();
        TIMER1.init();
    }
    info!("System initialized!");
    RTOS::spawn(device_executor, "Executor", 1024 * 4, 3);
    RTOS::start();
}

#[no_mangle]
pub extern "C" fn device_executor() {
    startup();
    Executor::run();
}

fn startup() {
    spawn!(command_prompt());
    LwipInterface::init(Some(debug_fn));
    WifiClient::init().unwrap_or_default();
    WifiClient::connect_wpa2("test", "test").unwrap();
    LwipInterface::dhcp(0);
    spawn!(http_server());
    spawn!(debug_task());
    TIMER1.start_periodical(100); //the fn on this is effectively a no-op, but it helps keep the main loop going
}

fn debug_fn(s: &str) {
    SERIAL1.tx_string(s);
}
