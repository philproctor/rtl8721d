use crate::prelude::*;
use core::cell::UnsafeCell;

pub struct SerialPort {
    initialized: UnsafeCell<bool>,
    tx: c::PinName,
    rx: c::PinName,
    baud: i32,
    parity: c::SerialParity,
    handle: UnsafeCell<c::serial_t>,
    rx_buffer: UnsafeCell<heapless::spsc::Queue<char, consts::U16>>,
}

unsafe impl Send for SerialPort {}
unsafe impl Sync for SerialPort {}

impl SerialPort {
    pub const fn new(tx: c::PinName, rx: c::PinName, baud: i32, parity: c::SerialParity) -> Self {
        Self {
            tx,
            rx,
            baud,
            parity,
            initialized: UnsafeCell::new(false),
            handle: UnsafeCell::new(c::serial_t {
                tx_len: 0,
                rx_len: 0,
                uart_idx: 0,
            }),
            rx_buffer: UnsafeCell::new(heapless::spsc::Queue(heapless::i::Queue::new())),
        }
    }

    unsafe extern "C" fn irq_rx(u: u32, irq: c::SerialIrq) {
        if irq == c::RxIrq {
            let zelf = Self::from_u32(u as usize);
            let c = c::serial_getc(&mut *zelf.handle.get());
            (&mut *zelf.rx_buffer.get())
                .enqueue((c as u8) as char)
                .unwrap_or_default();
        }
    }

    pub unsafe fn init(&self) {
        if *self.initialized.get() == false {
            let cptr = &mut *self.handle.get();
            c::serial_init(cptr, self.tx, self.rx);
            c::serial_baud(cptr, self.baud);
            c::serial_format(cptr, 8, self.parity, 1);
            c::serial_irq_handler(cptr, Some(Self::irq_rx), self.usize_addr() as u32);
            c::serial_irq_set(cptr, c::RxIrq, 1);
            *self.initialized.get() = true;
        }
    }

    fn usize_addr(&self) -> usize {
        unsafe { core::mem::transmute::<&Self, usize>(&self) }
    }

    unsafe fn from_u32(addr: usize) -> &'static Self {
        core::mem::transmute::<usize, &Self>(addr)
    }

    pub fn tx_string(&self, s: &str) {
        for c in s.chars() {
            self.tx(c);
        }
        self.tx('\r');
        self.tx('\n');
    }

    pub fn tx(&self, c: char) {
        unsafe {
            c::serial_putc(&mut *self.handle.get(), c as i32);
        }
    }

    pub async fn rx(&self) -> char {
        loop {
            match unsafe { &mut *self.rx_buffer.get() }.dequeue() {
                Some(c) => return c,
                None => yield_now().await,
            }
        }
    }

    pub async fn rx_line(&self, do_echo: bool) -> String {
        const MAX: usize = 128;
        let mut result = Vec::with_capacity(MAX / 4);
        loop {
            let c = self.rx().await;
            if do_echo {
                self.tx(c);
            }
            if matches!(c, '\n' | '\r') {
                return String::from_utf8(result).unwrap_or_else(|_| "".into());
            }
            result.push(c as u8);
            if result.len() >= MAX {
                return String::from_utf8(result).unwrap_or_else(|_| "".into());
            }
        }
    }
}

// pub static SERIAL1: SerialPort<{ c::PA_7 }, { c::PA_8 }, 115200, { c::ParityNone }> =
//     SerialPort::new();

// unsafe impl<
//         const TX: c::PinName,
//         const RX: c::PinName,
//         const BAUD: i32,
//         const PARITY: c::SerialParity,
//     > Send for SerialPort<TX, RX, BAUD, PARITY>
// {
// }

// unsafe impl<
//         const TX: c::PinName,
//         const RX: c::PinName,
//         const BAUD: i32,
//         const PARITY: c::SerialParity,
//     > Sync for SerialPort<TX, RX, BAUD, PARITY>
// {
// }

// pub struct SerialPort<
//     const TX: c::PinName,
//     const RX: c::PinName,
//     const BAUD: i32,
//     const PARITY: c::SerialParity,
// > {
//     buf: UnsafeCell<AtomicBuffer<char, 16>>,
//     handle: UnsafeCell<c::serial_t>,
// }

// impl<
//         const TX: c::PinName,
//         const RX: c::PinName,
//         const BAUD: i32,
//         const PARITY: c::SerialParity,
//     > SerialPort<TX, RX, BAUD, PARITY>
// {
//     const fn new() -> Self {
//         Self {
//             buf: UnsafeCell::new(AtomicBuffer::new([0u8 as char; 16])),
//             handle: UnsafeCell::new(c::serial_t {
//                 uart_idx: 0,
//                 rx_len: 0,
//                 tx_len: 0,
//             }),
//         }
//     }

//     unsafe extern "C" fn irq_rx(u: u32, irq: c::SerialIrq) {
//         if irq == c::RxIrq {
//             let zelf = Self::from_u32(u as usize);
//             let cptr = zelf.handle.get();
//             let c = c::serial_getc(cptr);
//             (&mut *zelf.buf.get()).write((c as u8) as char);
//         }
//     }

//     pub unsafe fn init(&self) {
//         let cptr = self.handle.get();
//         c::serial_init(cptr, TX, RX);
//         c::serial_baud(cptr, BAUD);
//         c::serial_format(cptr, 8, PARITY, 1);
//         c::serial_irq_handler(cptr, Some(Self::irq_rx), self.usize_addr() as u32);
//         c::serial_irq_set(cptr, c::RxIrq, 1);
//     }

//     fn usize_addr(&self) -> usize {
//         unsafe { core::mem::transmute::<&Self, usize>(&self) }
//     }

//     unsafe fn from_u32(addr: usize) -> &'static Self {
//         core::mem::transmute::<usize, &Self>(addr)
//     }

//     pub fn tx_string(&self, s: &str) {
//         for c in s.chars() {
//             self.tx(c);
//         }
//         self.tx('\r');
//         self.tx('\n');
//     }

//     pub fn tx(&self, c: char) {
//         unsafe {
//             c::serial_putc(self.handle.get(), c as i32);
//         }
//     }

//     pub async fn rx(&self) -> char {
//         loop {
//             let c = unsafe { &mut *self.buf.get() }.read();
//             match c {
//                 Some(c) => return c,
//                 None => yield_now().await,
//             }
//         }
//     }

//     pub async fn rx_line(&self, do_echo: bool) -> String {
//         let mut result = vec![];
//         loop {
//             let c = self.rx().await;
//             if do_echo {
//                 self.tx(c);
//             }
//             if matches!(c, '\n' | '\r') {
//                 return String::from_utf8(result).unwrap_or_else(|_| "".into());
//             }
//             result.push(c as u8);
//             if result.len() > 256 {
//                 return String::from_utf8(result).unwrap_or_else(|_| "".into());
//             }
//         }
//     }
// }
