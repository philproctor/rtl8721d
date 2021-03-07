use crate::ffi::ctypes::*;
use crate::ffi::*;
use core::alloc::{GlobalAlloc, Layout};

pub struct SystemAllocator;

unsafe impl GlobalAlloc for SystemAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System::alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System::dealloc(ptr, layout)
    }
}

pub struct System;

impl System {
    pub fn reset() -> ! {
        unsafe {
            c::sys_reset();
        }
        loop {}
    }

    pub fn now() -> u32 {
        unsafe { c::sys_now() }
    }

    pub fn ticks() -> u32 {
        unsafe { c::sys_jiffies() }
    }

    pub fn memory_available() -> usize {
        unsafe { c::xPortGetFreeHeapSize() }
    }

    pub fn sleep(ms: usize) {
        unsafe { c::sys_msleep(ms as u32) };
    }

    pub unsafe fn alloc(layout: Layout) -> *mut u8 {
        let res = c::pvPortMalloc(layout.size());
        return res as *mut u8;
    }

    pub unsafe fn dealloc(ptr: *mut u8, _layout: Layout) {
        c::vPortFree(ptr as *mut c_void);
    }
}
