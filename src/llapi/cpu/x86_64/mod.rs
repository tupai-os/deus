mod gdt;
mod idt;
mod isr;
mod exception;
mod irq;
mod port;
mod pic;

global_asm!(include_str!("isr.s"));

// Library
use spin::{Once, RwLock};

// Kernel
use crate::bootinfo::BootInfo;
use crate::llapi::cpu;

// TODO: Don't hard-code this
static VENDOR: &str = "Intel";

// Cpu

pub struct Cpu {
    info: cpu::CpuInfo,
    cores: [Core; 4],
}

impl Cpu {
    fn singleton() -> Self {
        Self {
            info: cpu::CpuInfo { vendor: VENDOR },
            cores: [
                Core::singleton(),
                Core::singleton(),
                Core::singleton(),
                Core::singleton(),
            ],
        }
    }
}

impl cpu::Cpu for Cpu {
    type Core = Core;

    fn info(&self) -> &cpu::CpuInfo { &self.info }
    fn cores(&self) -> &[Self::Core] { &self.cores }
    fn primary_core(&self) -> &Self::Core { &self.cores[0] }
    fn this_core(&self) -> &Self::Core { &self.cores[0] }
}

// Core

pub struct Core {
    info: cpu::CoreInfo,
}

impl Core {
    const fn singleton() -> Self {
        Self {
            info: cpu::CoreInfo,
        }
    }
}

impl cpu::Core for Core {
    fn info(&self) -> &cpu::CoreInfo { &self.info }
    unsafe fn enable_irqs(&self) { asm!("sti"); }
    unsafe fn disable_irqs(&self) { asm!("sti"); }
    fn await_irq(&self) { unsafe { asm!("hlt"); } }
}

// _start

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    idt::init();
    pic::init();
    exception::init();
    irq::init();

    crate::kernel_entry(BootInfo)
}

static SINGLETON: Once<Cpu> = Once::new();

pub fn singleton() -> &'static Cpu {
    SINGLETON.call_once(|| Cpu::singleton())
}
