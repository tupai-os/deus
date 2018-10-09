#![no_std]
#![no_main]

#![feature(asm)]

extern crate bootloader_precompiled;

use core::panic::PanicInfo;

static BOOT_MSG: &[u8] = b"Deus started.";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    BOOT_MSG.iter().enumerate().for_each(|(i, &b)| unsafe {
        *vga_buffer.offset(i as isize * 2) = b;
        *vga_buffer.offset(i as isize * 2 + 1) = 0x0F;
    });

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
