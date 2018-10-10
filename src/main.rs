#![no_std]
#![no_main]

#![feature(asm)]

pub mod dev;
pub mod driver;

// Library
use core::panic::PanicInfo;

// Kernel
use crate::dev::SerialWriter;
use crate::driver::vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Display a welcome message
    vga::SINGLETON
        .write()
        .textmode_mut()
        .write_many(b"Deus started.".iter().map(|&b| b.into()))
        .unwrap();

    loop {}
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    // Forcibly take the VGA lock - showing the error is more important than safety at this point!
    vga::SINGLETON.force_write_unlock();

    // Display the panic message
    vga::SINGLETON
        .write()
        .textmode_mut()
        .write_many(b"[KERNEL PANIC]".iter().map(|&b| b.into()))
        .unwrap();

	loop {}
}
