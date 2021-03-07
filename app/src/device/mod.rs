use crate::hal::*;
use crate::prelude::*;
pub use i::*;

#[cfg(not(test))]
mod hooks;

#[cfg(test)]
mod test_harness;

#[cfg(not(test))]
mod i {
    use super::*;
    pub static SERIAL1: SerialPort = SerialPort::new(c::PA_7, c::PA_8, 115200, c::ParityNone);
    pub static TIMER1: Timer = Timer::new(3, None);
}

#[cfg(test)]
mod i {
    use super::*;
    pub static SERIAL1: SerialPort = SerialPort::new(c::PA_7, c::PA_8, 115200, c::ParityNone);
    pub static TIMER1: Timer = Timer::new(3, None);
}

// fn testfn() {
//     info!("testfn()");
// }
