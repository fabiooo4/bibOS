#![no_std] // Remove std
#![no_main] // Remove main

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello, World!";

// Overwrite the default entry point function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        let i: isize = i as isize * 2;
        unsafe {
            *vga_buffer.offset(i) = byte;
            *vga_buffer.offset(i + 1) = 0xb;
        }
    }

    panic!()
}

// Define a custom panic handler that does not return
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
