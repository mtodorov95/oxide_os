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
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

mod vga_buffer;

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Preserves the actual name of the fundtion when going through the compiler
// extern "C" - use the C calling convention instead
// Linker looks for fn named _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
