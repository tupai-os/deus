// Library
use spin::{RwLock, Once};

// Kernel
use crate::llapi::cpu;

// TODO: Don't hard-code this
static VENDOR: &str = "Intel";

// Cpu

pub struct Cpu {
    info: cpu::CpuInfo,
    cores: [Core; 1],
}

impl Cpu {
    fn singleton() -> Self {
        Self {
            info: cpu::CpuInfo {
                vendor: VENDOR,
            },
            cores: [Core::singleton(); 1]
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
}

static SINGLETON: Once<RwLock<Cpu>> = Once::new();

pub fn singleton() -> &'static RwLock<Cpu> {
    SINGLETON.call_once(|| RwLock::new(Cpu::singleton()))
}
