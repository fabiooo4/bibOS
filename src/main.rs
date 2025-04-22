#![allow(clippy::empty_loop)]
#![no_std] // Remove std
#![no_main] // Remove main

// Setup a custom test runner
#![feature(custom_test_frameworks)]
#![test_runner(bib_os::test_runner)]
// Change test function name to allow calling from _start
#![reexport_test_harness_main = "test_main"]

use bib_os::{
    allocator, hlt_loop, init,
    memory::{self, BootInfoFrameAllocator},
    println,
};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

entry_point!(kernel_main);

// Overwrite the default entry point function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();

    println!("Hello, World{}", "!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value ({}) at {:p}", *heap_value, heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec {:?}... at {:p}", &vec[0..5], vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {} for vec: {:?}",
        Rc::strong_count(&cloned_reference),
        *reference_counted
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

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
