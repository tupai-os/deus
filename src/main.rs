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
use crate::llapi::mach::traits::{Core, Cpu};

/// The platform-agnostic kernel entry point
///
/// At this stage in the boot process, the machine should be in a stable
/// condition with IRQs disabled (but ready to be enabled), the kernel heap
/// initiated and logging enabled.
pub fn kernel_entry(_info: BootInfo) -> ! {
    // We use a scope to ensure everything gets dropped before the scheduler starts
    {
        // Display a welcome message
        logln!("Deus started.");

        // Display CPU information
        logln!("{}", llapi::mach::cpu::singleton().info());

        // Enable IRQs
        unsafe { llapi::mach::cpu::singleton().primary_core().enable_irqs(); }
    }

    loop {
        llapi::mach::cpu::singleton().this_core().await_irq();
    }
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    logln!("[PANIC] {}", info);
    loop {
        // Disable IRQs and halt
        for core in llapi::mach::cpu::singleton().cores() {
            unsafe { core.disable_irqs(); }
            core.await_irq();
        }
    }
}
