#![no_std]

extern crate alloc;
mod common;
mod util;

//Common stuff
pub use common::*;
pub use util::MemoryStream;
pub use embedded_nal::{AddrType, Dns, IpAddr, SharableStack, TcpClientStack, TcpFullStack};

use alloc::string::String;

// Client feature..
mod client;
pub use client::*;
pub trait ClientStack: TcpClientStack + Dns + Clone + Send {}
impl<T: TcpClientStack + Dns + Clone + Send> ClientStack for T {}

// Server feature..
mod server;
pub use server::*;
pub trait ServerStack: TcpFullStack + Clone {}
impl<T: TcpFullStack + Clone> ServerStack for T {}

pub struct Http<T: ClientStack + ServerStack> {
    stack: T,
}

impl<T: ClientStack + ServerStack> Http<T> {
    pub fn new(stack: T) -> Self {
        Self { stack }
    }
}
