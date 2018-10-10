#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// Suppress warnings when testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![feature(asm)]

pub mod dev;
pub mod vdev;
pub mod driver;

// Library
use core::panic::PanicInfo;

// Kernel
use crate::dev::Printer;
use crate::vdev::tty;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Display a welcome message
    tty::default_do_for_mut(|tty| {
        tty.write_str("Deus started.").unwrap();
    }).unwrap();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    // Forcibly take the VGA lock - showing the error is more important than safety at this point!
    // TODO: Make a driver-agnostic way of doing this
    use crate::driver::vga;
    vga::singleton().force_write_unlock();

    // Display the panic message
    tty::default_do_for_mut(|tty| {
        tty.write_str("[KERNEL PANIC]").unwrap();
    }).unwrap();

	loop {}
}
