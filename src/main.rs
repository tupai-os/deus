#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// Suppress warnings when testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![feature(asm, self_struct_ctor)]

pub mod driver;
pub mod llapi;
pub mod util;
pub mod vdev;

#[macro_use]
pub mod log;

// Library
use core::panic::PanicInfo;

// Kernel
use crate::llapi::cpu::{Cpu, Core};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Display a welcome message
    logln!("Deus started.");

    logln!("{}", llapi::cpu::singleton().read().info());
    for (i, core) in llapi::cpu::singleton().read().cores().iter().enumerate() {
        logln!("Core {}:", i);
        logln!("{}", core.info());
    }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    logln!("[PANIC] {}", info);
	loop {}
}
