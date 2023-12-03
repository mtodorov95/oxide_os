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

use core::panic::PanicInfo;
use oxide_os::println;

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

// Preserves the actual name of the fundtion when going through the compiler
// extern "C" - use the C calling convention instead
// Linker looks for fn named _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    oxide_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page atble at: {:?}", level_4_page_table.start_address());

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
