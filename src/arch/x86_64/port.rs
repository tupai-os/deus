use core::{marker::PhantomData, arch::asm};
use spin::Mutex;
use crate::util::wait_cycles;

const DELAY_CYCLES: usize = 150;

pub unsafe fn out8(port: u16, val: u8) {
    ::x86::io::outb(port, val)
}

pub unsafe fn in8(port: u16) -> u8 {
    wait_cycles(DELAY_CYCLES);
    ::x86::io::inb(port)
}

// PortLock

pub trait Port {
    const ADDR: u16;
}

// TODO: Improve this when const generics come
pub struct PortLock<P: Sized> {
    lock: Mutex<()>,
    _phantom: PhantomData<P>,
}

impl<P: Sized> PortLock<P> {
    pub const fn new() -> Self {
        Self {
            lock: Mutex::new(()),
            _phantom: PhantomData,
        }
    }
}

impl<P: Port + Sized> PortLock<P> {
    pub fn read(&self) -> u8 {
        let _ = self.lock.lock();
        unsafe { in8(P::ADDR) }
    }

    pub fn write(&self, val: u8) {
        let _ = self.lock.lock();
        unsafe { out8(P::ADDR, val) }
    }
}
