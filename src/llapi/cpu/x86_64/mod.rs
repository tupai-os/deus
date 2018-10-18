mod gdt;
mod idt;
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

    fn info(&self) -> &cpu::CpuInfo {
        &self.info
    }
    fn cores(&self) -> &[Self::Core] {
        &self.cores
    }
    fn cores_mut(&mut self) -> &mut [Self::Core] {
        &mut self.cores
    }
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
    fn info(&self) -> &cpu::CoreInfo {
        &self.info
    }
    unsafe fn enable_irqs(&self) {
        asm!("sti");
    }
    unsafe fn disable_irqs(&self) {
        asm!("sti");
    }
}

// _start

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    idt::init();
    exception::init();
    irq::init();
    pic::init();

    crate::kernel_entry(BootInfo)
}

static SINGLETON: Once<RwLock<Cpu>> = Once::new();

pub fn singleton() -> &'static RwLock<Cpu> {
    SINGLETON.call_once(|| RwLock::new(Cpu::singleton()))
}
