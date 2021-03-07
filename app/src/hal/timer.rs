use crate::prelude::*;
use core::cell::UnsafeCell;

pub struct Timer {
    handle: UnsafeCell<c::gtimer_t>,
    callback: Option<fn()>,
}

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

impl Timer {
    pub const fn new(tid: u8, f: Option<fn()>) -> Self {
        Self {
            handle: UnsafeCell::new(c::gtimer_t {
                handler: core::ptr::null_mut(),
                hid: 0,
                timer_id: tid,
                is_periodcal: 0,
            }),
            callback: f,
        }
    }

    extern "C" fn handler(hid: u32) {
        let zelf: &Self = unsafe { core::mem::transmute(hid) };
        if let Some(f) = zelf.callback {
            f();
        }
    }

    pub unsafe fn init(&self) {
        (&mut *self.handle.get()).handler = Self::handler as *mut c_void;
        c::gtimer_init(self.handle.get(), 0);
    }

    pub fn start_periodical(&self, duration: u32) {
        info!("Starting periodic timer with duration {}us", duration);
        unsafe {
            c::gtimer_start_periodical(
                self.handle.get(),
                duration,
                Self::handler as *mut c_void,
                core::mem::transmute(self),
            )
        };
    }

    pub fn start_one_shout(&self, duration: u32) {
        unsafe {
            c::gtimer_start_one_shout(
                self.handle.get(),
                duration,
                Self::handler as *mut c_void,
                core::mem::transmute(self),
            )
        };
    }

    pub fn start(&self) {
        unsafe { c::gtimer_start(self.handle.get()) };
    }

    pub fn stop(&self) {
        unsafe { c::gtimer_stop(self.handle.get()) };
    }

    pub fn read_tick(&self) -> u32 {
        unsafe { c::gtimer_read_tick(self.handle.get()) }
    }

    pub fn read_us(&self) -> u64 {
        unsafe { c::gtimer_read_us(self.handle.get()) }
    }
}
