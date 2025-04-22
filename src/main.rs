#![allow(clippy::empty_loop)]
#![no_std] // Remove std
#![no_main] // Remove main

// Setup a custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(bib_os::test_runner)]
// Change test function name to allow calling from _start
#![reexport_test_harness_main = "test_main"]

use bib_os::{
    hlt_loop, init,
    memory::{self},
    println,
};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::{VirtAddr, structures::paging::Translate};

entry_point!(kernel_main);

// Overwrite the default entry point function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();

    println!("Hello, World{}", "!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash");

    hlt_loop();
}

// Define a custom panic handler that does not return
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use bib_os::eprint;

    eprint!("\n{info}");
    hlt_loop();
}

// Define a test panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bib_os::test_panic_handler(info)
}
