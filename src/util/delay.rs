use crate::hal::RTOS;
pub use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

///Use RTOS provided delay
#[derive(Debug)]
pub struct Delay {
    started: u32,
    duration: u32,
}

impl Unpin for Delay {}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        let cur = RTOS::ms_elapsed();
        let elapsed = cur - self.started;
        if elapsed > self.duration {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[allow(dead_code)]
impl Delay {
    pub fn new(ms: u32) -> Self {
        Delay {
            started: RTOS::ms_elapsed(),
            duration: ms,
        }
    }
}
