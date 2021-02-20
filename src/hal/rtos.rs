use crate::ffi::{ctypes::*, *};
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};

pub struct RtosAllocator;

unsafe impl GlobalAlloc for RtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        RTOS::alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        RTOS::dealloc(ptr, layout)
    }
}

pub struct RTOS;

impl RTOS {
    pub fn do_yield() {
        unsafe {
            c::vPortYield();
        }
    }

    pub fn ticks_elapsed() -> u32 {
        unsafe { c::xTaskGetTickCount() }
    }

    pub fn ms_elapsed() -> u32 {
        unsafe { c::xTaskGetTickCount() } // ticks == ms on this system, but this should be done in a more portable way
    }

    pub unsafe fn alloc(layout: Layout) -> *mut u8 {
        let res = c::pvPortMalloc(layout.size() as u32);
        return res as *mut u8;
    }

    pub unsafe fn dealloc(ptr: *mut u8, _layout: Layout) {
        c::vPortFree(ptr as *mut c_void);
    }

    pub fn start() -> ! {
        unsafe { c::vTaskStartScheduler(); }
        unreachable!()
    }

    pub fn spawn(f: extern "C" fn(), name: &'static str, stack_size: u16, priority: u32) {
        let ptr = f as *const ();
        unsafe {
            let mut c_name: Vec<u8> = name.as_bytes().into();
            c_name.push(0u8);
            let f_ptr =
                core::mem::transmute::<*const (), unsafe extern "C" fn(arg1: *mut c_void)>(ptr);
            c::xTaskCreate(
                Some(f_ptr),
                c_name.as_ptr(),
                stack_size,
                &mut *core::ptr::null_mut(),
                priority,
                &mut *core::ptr::null_mut(),
            );
        }
    }
}
