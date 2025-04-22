#![no_std] // Remove std
#![no_main] // Remove main

// Setup a custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(bib_os::test_runner)]
// Change test function name to allow calling from _start
#![reexport_test_harness_main = "test_main"]

use bib_os::{eprintln, hlt_loop, println};
use core::panic::PanicInfo;

#[allow(clippy::empty_loop)]
#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bib_os::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[test_case]
fn test_erintln() {
    eprintln!("test_println output");
}
