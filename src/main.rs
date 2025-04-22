#![no_std] // Remove std
#![no_main] // Remove main

// Setup a custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(bib_os::test_runner)]
// Change test function name to allow calling from _start
#![reexport_test_harness_main = "test_main"]

use bib_os::println;
use core::panic::PanicInfo;

// Overwrite the default entry point function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    #[cfg(test)]
    test_main();

    panic!("Kernel terminated")
}

// Define a custom panic handler that does not return
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use bib_os::eprint;

    eprint!("\n{info}");
    loop {}
}

// Define a test panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bib_os::test_panic_handler(info)
}
