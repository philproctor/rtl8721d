use crate::prelude::*;
use core::alloc::Layout;
use core::panic::PanicInfo;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[global_allocator]
static ALLOCATOR: RtosAllocator = RtosAllocator;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

pub fn runner(tests: &[&dyn Fn()]) {
    // println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

// #[cfg(test)]
// pub extern "C" fn _start() -> ! {
//     // println!("Hello World{}", "!");

//     #[cfg(test)]
//     crate::test_main();

//     loop {}
// }

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}
