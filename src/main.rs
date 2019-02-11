#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// Suppress warnings when testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![feature(asm, global_asm)]

#[macro_use]
extern crate lazy_static;

pub mod arch;
pub mod driver;
pub mod util;
pub mod vdev;

#[macro_use]
pub mod log;
pub mod bootinfo;

// Library
use core::panic::PanicInfo;

// Kernel
use crate::bootinfo::BootInfo;

/// The platform-agnostic kernel start point
///
/// At this stage in the boot process, the machine should be in a stable
/// condition with IRQs disabled (but ready to be enabled), the kernel heap
/// initiated and logging enabled.
pub fn kernel_start(_info: BootInfo) -> ! {
    // Display a welcome message
    logln!("Deus started.");

    // Display CPU information
    logln!("{}", arch::cpu::active::vendor());

    // Enable IRQs
    unsafe { arch::cpu::active::enable_irqs(0); }

    loop {
        arch::cpu::active::await_irqs(0);
    }
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    logln!("[PANIC] {}", info);
    loop {
        unsafe {
            arch::cpu::active::disable_irqs(0);
            arch::cpu::active::await_irqs(0);
        }
    }
}
