pub mod gdt;
pub mod idt;
pub mod isr;
pub mod fault;
pub mod port;

use core::arch::asm;
use bootloader::entry_point;
use crate::{hw, BootInfo};

entry_point!{start}

fn start(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    idt::init();
    fault::init();

    hw::x86::init();

    crate::kernel_start(boot_info)
}

pub mod hal {
    use super::*;

    // Reexports
    pub use isr::StackFrame;

    pub unsafe fn enable_irqs() {
        asm!("sti");
    }

    pub unsafe fn disable_irqs() {
        asm!("cli");
    }

    pub fn await_irqs() {
        unsafe { asm!("hlt"); }
    }

    pub fn irqs_enabled() -> bool {
        let val: u64;
        unsafe { asm!("pushf; pop {0}", out(reg) val); }
        val & (1 << 9) > 0
    }
}
