// Compile for custom target (host by default) that doesn't have an
// underlying OS. That way the linker won't assume that there is an underlying
// C runtime.
// cargo build --target x86_64-oxide_os.json

// Don't link the standard library
#![no_std]
// Overwrite the standard entry point chain e.g. crt0 -> start > main
#![no_main]
// Enable testing in a no std environment
#![feature(custom_test_frameworks)]
#![test_runner(oxide_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::{boxed::Box, vec, rc::Rc, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use oxide_os::{allocator, memory::BootInfoFrameAllocator, println};

// Called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    oxide_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oxide_os::test_panic_handler(info)
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use oxide_os::memory;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    oxide_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let ref_counted = Rc::new(vec![1,2,3]);
    let cloned_ref = ref_counted.clone();
    println!("current ref count is {}", Rc::strong_count(&cloned_ref));
    core::mem::drop(ref_counted);
    println!("ref count is {} now", Rc::strong_count(&cloned_ref));

    #[cfg(test)]
    test_main();
    println!("Didn't crash");
    // Halt the CPU instead of running at 100% all the time
    oxide_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
