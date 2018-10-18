// Library
use core::fmt;

// Cpu

pub trait Cpu {
    type Core: Core;

    fn info(&self) -> &CpuInfo;
    fn cores(&self) -> &[Self::Core];
    fn cores_mut(&mut self) -> &mut [Self::Core];
}

// CpuInfo

pub struct CpuInfo {
    vendor: &'static str,
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
        CPU Information\n\
        ---------------\n\
        Vendor: {}\n\
        ", self.vendor)
    }
}

// Core

pub trait Core {
    fn info(&self) -> &CoreInfo;
}

// CoreInfo

pub struct CoreInfo;

impl fmt::Display for CoreInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
        Core Information\n\
        ---------------\n\
        ")
    }
}

// TODO: Cfg-gate this
pub mod x86_64;
pub use self::x86_64::*;