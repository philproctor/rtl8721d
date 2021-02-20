#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(lang_items)]
#![feature(try_trait)]
#![feature(associated_type_bounds)]
#![allow(unused_attributes)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_harness::runner)]
// #![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[macro_use]
extern crate num_derive;

#[macro_use]
mod log;

#[cfg(test)]
mod test_harness;

#[cfg(not(test))]
mod hw;

pub mod ffi;
pub mod prelude;

mod error;
mod executor;
mod hal;
mod tasks;
mod util;

use prelude::*;
use tasks::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        Executor::init();
        hal::init();
    }
    hal::SERIAL1.tx_string("Starting system...");
    RTOS::spawn(device_executor, "Executor", 2048, 3);
    Executor::spawn(command_prompt());
    RTOS::start();
}

#[no_mangle]
pub extern "C" fn device_executor() {
    Executor::run();
}
