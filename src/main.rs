#![no_std] // Remove std
#![no_main] // Remove main

use core::panic::PanicInfo;

// Overwrite the default entry point function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    panic!();
}

// Define a custom panic handler that does not return
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
