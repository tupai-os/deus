#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// Suppress warnings when testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![feature(asm)]

pub mod dev;
pub mod vdev;
pub mod driver;

#[macro_use]
pub mod log;

// Library
use core::panic::PanicInfo;

// Kernel
use crate::vdev::tty;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Display a welcome message
    logln!("Deus started.");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    logln!("[PANIC] {}", info);
	loop {}
}
