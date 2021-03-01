use crate::ffi::ctypes::*;
use crate::prelude::*;
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
        unsafe {
            c::vTaskStartScheduler();
        }
        unreachable!()
    }

    pub fn spawn(f: extern "C" fn(), name: &'static str, stack_size: u16, priority: u32) {
        let ptr = f as *const ();
        let cstr = CString::new(name);
        unsafe {
            let f_ptr =
                core::mem::transmute::<*const (), unsafe extern "C" fn(arg1: *mut c_void)>(ptr);
            c::xTaskCreate(
                Some(f_ptr),
                cstr.ptr(),
                stack_size,
                &mut *core::ptr::null_mut(),
                priority,
                &mut *core::ptr::null_mut(),
            );
        }
    }
}

pub struct RtosQueue<T> {
    handle: c::QueueHandle_t,
    phantom: core::marker::PhantomData<T>,
}

unsafe impl<T> Send for RtosQueue<T> {}
unsafe impl<T> Sync for RtosQueue<T> {}

impl<T> RtosQueue<T> {
    pub fn new(size: usize) -> Result<Self> {
        // #define queueQUEUE_TYPE_BASE				( ( uint8_t ) 0U )
        // #define queueQUEUE_TYPE_SET					( ( uint8_t ) 0U )
        // #define queueQUEUE_TYPE_MUTEX 				( ( uint8_t ) 1U )
        // #define queueQUEUE_TYPE_COUNTING_SEMAPHORE	( ( uint8_t ) 2U )
        // #define queueQUEUE_TYPE_BINARY_SEMAPHORE	( ( uint8_t ) 3U )
        // #define queueQUEUE_TYPE_RECURSIVE_MUTEX		( ( uint8_t ) 4U )
        Ok(Self {
            handle: unsafe {
                c::xQueueGenericCreate(size as u32, core::mem::size_of::<T>() as u32, 0)
            },
            phantom: core::marker::PhantomData,
        })
    }

    pub async fn pop(&self) -> T {
        loop {
            match self.try_pop() {
                Some(t) => return t,
                None => yield_now().await,
            }
        }
    }

    pub fn try_pop(&self) -> Option<T> {
        let mut target: T = unsafe { core::mem::zeroed() };
        let target_ptr = &mut target as *mut T;
        let res =
            unsafe { c::xQueueReceive(self.handle, core::mem::transmute(target_ptr), 0) } == 1;
        if res {
            Some(target)
        } else {
            None
        }
    }

    pub async fn push(&self, val: T) {
        loop {
            match self.try_push(&val) {
                Ok(_) => return,
                Err(_) => yield_now().await,
            }
        }
    }

    pub fn try_push(&self, val: &T) -> Result<()> {
        // #define	queueSEND_TO_BACK		( ( BaseType_t ) 0 )
        // #define	queueSEND_TO_FRONT		( ( BaseType_t ) 1 )
        // #define  queueOVERWRITE			( ( BaseType_t ) 2 )
        let val_ptr = val as *const T;
        let res =
            unsafe { c::xQueueGenericSend(self.handle, core::mem::transmute(val_ptr), 0, 0) } == 1;
        if res {
            Ok(())
        } else {
            Err(SystemError::Unknown)
        }
    }
}
