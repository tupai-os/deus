// Library
use core::fmt;

// Cpu

pub trait Cpu {
    type Core: Core;

    fn info(&self) -> &CpuInfo;
    fn cores(&self) -> &[Self::Core];
    fn primary_core(&self) -> &Self::Core;
    fn this_core(&self) -> &Self::Core;
}

// CpuInfo

pub struct CpuInfo {
    pub vendor: &'static str,
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
             CPU Information\n\
             ---------------\n\
             Vendor: {}\n\
             ",
            self.vendor
        )
    }
}

// Core

pub trait Core {
    fn info(&self) -> &CoreInfo;
    unsafe fn enable_irqs(&self);
    unsafe fn disable_irqs(&self);
    fn await_irq(&self);
}

// CoreInfo

pub struct CoreInfo;

impl fmt::Display for CoreInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
             Core Information\n\
             ---------------\n\
             "
        )
    }
}
