#![no_std] // Remove std
#![no_main] // Remove main

mod vga_buffer;

// Overwrite the default entry point function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    panic!("Custom panic message")
}

// Define a custom panic handler that does not return
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprint!("\n{info}");
    loop {}
}
