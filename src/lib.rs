#![allow(clippy::empty_loop)]
#![no_std] // Remove std
#![cfg_attr(test, no_main)]
// Setup a custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// Change test function name to allow calling from _start
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga_buffer;
use core::{fmt, panic::PanicInfo};

// Lib test setup
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
// ----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("{}", Green("[ok]"));
    }
}

// Custom test panic handler
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}\n", Red("[failed]"));
    serial_println!("{} {}\n", Red("Error:"), info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Custom test runner
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\nRunning {} tests", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!();

    exit_qemu(QemuExitCode::Success);
}

// Colored text for serial output
pub struct Green(pub &'static str);
impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[32m")?; // prefix code
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?; // postfix code
        Ok(())
    }
}

pub struct Red(pub &'static str);

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[31m")?; // prefix code
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?; // postfix code
        Ok(())
    }
}
