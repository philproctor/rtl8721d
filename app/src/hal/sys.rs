use crate::ffi::*;

pub struct System;

impl System {
    pub fn reset() -> ! {
        unsafe {
            c::sys_reset();
        }
        unreachable!()
    }

    pub fn now() -> u32 {
        unsafe { c::sys_now() }
    }

    pub fn ticks() -> u32 {
        unsafe { c::sys_jiffies() }
    }

    pub fn memory_available() -> u32 {
        unsafe { c::xPortGetFreeHeapSize() }
    }

    pub fn sleep(ms: usize) {
        unsafe { c::sys_msleep(ms as u32) };
    }
}
