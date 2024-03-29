#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// Suppress warnings when testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![feature(
    alloc_error_handler,
    exclusive_range_pattern,
    naked_functions,
)]

#[macro_use]
extern crate alloc;

mod arch;
mod hw;
mod driver;
mod mem;
mod util;
mod vdev;
mod sched;
mod ipc;
mod iface;

#[macro_use]
mod log;
mod startup;

// Reexports
pub use arch::hal;

use core::{
    panic::PanicInfo,
    mem::MaybeUninit,
    cell::UnsafeCell,
};
use bootloader::BootInfo;
use crate::mem::Heap;

#[global_allocator]
static mut GLOBAL_ALLOCATOR: Heap = Heap::uninit();
static mut HEAP: UnsafeCell<[u8; 65536 * 8]> = UnsafeCell::new([0u8; 65536 * 8]);

static mut BOOT_INFO: MaybeUninit<&'static BootInfo> = MaybeUninit::uninit();

pub fn boot_info() -> &'static BootInfo {
    unsafe { BOOT_INFO.assume_init_ref() }
}

/// The platform-agnostic kernel start point
///
/// At this stage in the boot process, the machine should be in a stable
/// condition with IRQs disabled (but ready to be enabled), the kernel heap
/// initiated and logging enabled.
pub fn kernel_start(info: &'static BootInfo) -> ! {
    // Register boot info
    unsafe { BOOT_INFO.write(info); }

    logln!("BootInfo is as {:?}", boot_info() as *const _);

    // Init heap
    unsafe { GLOBAL_ALLOCATOR.init(HEAP.get_mut()); }

    // Initiate subsystems
    ipc::init_exchange();
    sched::init_threading();
    startup::init_drivers();

    for channel in ipc::exchange().list() {
        logln!(":: {}", channel);
    }

    sched::spawn_thread(|| {
        let rx = ipc::exchange()
            .connect_rx::<iface::Char>("kbd")
            .unwrap();

        while let Ok(c) = rx.recv() {
            use core::convert::TryFrom;
            log!("{}", char::try_from(c.0).unwrap());
        }
    });

    //assert!(!hal::irqs_enabled());
    unsafe { hal::enable_irqs(); }
    loop { hal::await_irqs(); }
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    logln!("[PANIC] {}", info);
    loop {
        unsafe {
            hal::disable_irqs();
            hal::await_irqs();
        }
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Failed to allocate {:?}", layout);
}
