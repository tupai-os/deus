pub mod gdt;
pub mod idt;
pub mod isr;
pub mod fault;
pub mod port;

use crate::{
    bootinfo::BootInfo,
    arch::family,
};

pub struct Ty;

pub fn vendor() -> &'static str {
    "Intel"
}

pub unsafe fn enable_irqs(core: usize) {
    asm!("sti");
}

pub unsafe fn disable_irqs(core: usize) {
    asm!("cli");
}

pub fn await_irqs(core: usize) {
    unsafe { asm!("hlt"); }
}


#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    idt::init();
    fault::init();

    family::active::init_hw();

    crate::kernel_start(BootInfo)
}
