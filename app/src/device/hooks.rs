use crate::prelude::*;
use core::alloc::Layout;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: SystemAllocator = SystemAllocator;

#[lang = "eh_personality"]
extern "C" fn eh_personality() -> ! {
    System::reset();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(pi: &PanicInfo) -> ! {
    crit!("PANIC!!! {:?}", pi);
    System::sleep(5000);
    System::reset();
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    System::reset();
}

#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
pub fn __aeabi_dcmple() {}
