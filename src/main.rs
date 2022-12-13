#![no_std] // don't link the standard library
#![no_main] // disable rust level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::println;

// Macro to provide type checked way to use Rust function as entry point
entry_point!(kernel_main);

// Overwrite OS entry point
// disables name mangling as we need to know
// the exact name to pass to the linker
// extern C tells compiler to use C calling convention
// ! is diverging function, does not return
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::{active_level_4_table, translate_addr};
    use x86_64::VirtAddr;

    println!("Hell World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let address = [
        // identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        //stack page
        0x0100_0020_1a10,
        // virt addr mapped to phys addr 0
        boot_info.physical_memory_offset,
    ];

    for &address in &address {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It didn't crash!");
    rust_os::hlt_loop();
}

// Called on panic
// ! is 'never' type, does not return
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
