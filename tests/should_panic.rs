#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use bib_os::{exit_qemu, hlt_loop, serial_print, serial_println, Green, QemuExitCode, Red};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("{}", Red("[test did not panic]"));
    exit_qemu(QemuExitCode::Failed);

    hlt_loop()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", Green("[ok]"));
    exit_qemu(QemuExitCode::Success);
    hlt_loop()
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
