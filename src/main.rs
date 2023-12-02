// Add and compile for target (host by default) that doesn't have an
// underlying OS. That way the linker won't assume that there is an underlying
// C runtime.
// The following is an embedded ARM system 
// rustup target add thumbv7em-none-eabihf
// cargo build --target thumbv7em-none-eabihf


#![no_std]
// Overwrite the standard entry point chain e.g. crt0 -> start > main
#![no_main]

use core::panic::PanicInfo;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Preserves the actual name of the fundtion when going through the compiler
// extern "C" - use the C calling convention instead
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
