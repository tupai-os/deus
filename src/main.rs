#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// Suppress warnings when testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![feature(asm, self_struct_ctor, global_asm)]

pub mod driver;
pub mod llapi;
pub mod util;
pub mod vdev;

#[macro_use]
pub mod log;
pub mod bootinfo;

// Library
use core::panic::PanicInfo;

// Kernel
use crate::bootinfo::BootInfo;
use crate::llapi::cpu::{Core, Cpu};

/// The platform-agnostic kernel entry point
///
/// At this stage in the boot process, the machine should be in a stable
/// condition with IRQs disabled, the kernel heap initiated and logging
/// enabled.
pub fn kernel_entry(_info: BootInfo) -> ! {
    // Display a welcome message
    logln!("Deus started.");

    // Display CPU information
    logln!("{}", llapi::cpu::singleton().read().info());

    // Enable IRQs
    unsafe { llapi::cpu::singleton().read().cores()[0].enable_irqs(); }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    // Disable IRQs
    unsafe { llapi::cpu::singleton().read().cores().iter().for_each(|core| core.disable_irqs()); }
    logln!("[PANIC] {}", info);
    loop {}
}
