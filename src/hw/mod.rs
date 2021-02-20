use crate::prelude::*;
use core::alloc::Layout;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: RtosAllocator = RtosAllocator;

#[lang = "eh_personality"]
extern "C" fn eh_personality() -> ! {
    System::reset();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    System::reset();
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    System::reset();
}
