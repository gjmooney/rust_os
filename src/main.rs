#![no_std] // don't link the standard library
#![no_main] // disable rust level entry points

use core::panic::PanicInfo;

mod vga_buffer;


// Overwrite OS entry point
// disables name mangling as we need to know
// the exact name to pass to the linker
// extern C tells compiler to use C calling convention
// ! is divergin funtion, does not return
#[no_mangle] 
pub extern "C" fn _start() -> ! {
    println!("Hell World{}", "!");

    loop {}
}

// Called on panic
// ! is 'never' type, does not return
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

