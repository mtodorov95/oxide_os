// Compile for custom target (host by default) that doesn't have an
// underlying OS. That way the linker won't assume that there is an underlying
// C runtime.
// cargo build --target x86_64-oxide_os.json

// Don't link the standard library
#![no_std]
// Overwrite the standard entry point chain e.g. crt0 -> start > main
#![no_main]

use core::panic::PanicInfo;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// Preserves the actual name of the fundtion when going through the compiler
// extern "C" - use the C calling convention instead
// Linker looks for fn named _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Get raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // Write the string byte and color byte (0xb) to the vga_buffer
        // Unsafe, because Rust can't prove that a pointer we created is valid
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
