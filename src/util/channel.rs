use crate::prelude::*;
use alloc::collections::VecDeque;

#[derive(Debug)]
pub struct Channel<T> {
    c: Mutex<VecDeque<T>>,
}

unsafe impl<T> Sync for Channel<T> {}
unsafe impl<T> Send for Channel<T> {}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self {
            c: Mutex::new(VecDeque::new()),
        }
    }
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            c: Mutex::new(Default::default()),
        }
    }

    pub unsafe fn irq_send(&self, val: T) {
        self.c.bypass_lock().push_back(val);
    }

    pub fn blocking_send(&self, val: T) {
        let mut c = self.c.spin_lock();
        (&mut c).push_back(val);
        return;
    }

    pub async fn send(&self, val: T) {
        let mut c = self.c.lock().await;
        (&mut c).push_back(val);
        return;
    }

    pub fn blocking_recv(&self) -> Option<T> {
        let mut c = self.c.spin_lock();
        return (&mut c).pop_front();
    }

    pub async fn recv(&self) -> Option<T> {
        let mut c = self.c.lock().await;
        return (&mut c).pop_front();
    }
}
