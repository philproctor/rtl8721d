#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(safe_packed_borrows)]
#![allow(improper_ctypes)]

pub mod bindings;
pub mod ctypes;

pub use bindings as c;
