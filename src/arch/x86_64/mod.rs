pub mod gdt;
pub mod idt;
pub mod isr;
pub mod fault;
pub mod port;

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
        asm!("sti" :::: "volatile");
    }

    pub unsafe fn disable_irqs() {
        asm!("cli" :::: "volatile");
    }

    pub fn await_irqs() {
        unsafe { asm!("hlt" :::: "volatile"); }
    }

    pub fn irqs_enabled() -> bool {
        let val: u64;
        unsafe { asm!("pushf; pop %rax" : "=r{rax}"(val) :: "rax" : "volatile"); }
        val & (1 << 9) > 0
    }
}
