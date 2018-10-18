// Library
use core::marker::PhantomData;
use spin::Mutex;

// Kernel
use crate::util::wait_cycles;

const DELAY_CYCLES: usize = 150;

unsafe fn out8(port: u16, val: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val) :: "volatile");
    wait_cycles(DELAY_CYCLES);
}

unsafe fn in8(port: u16) -> u8 {
    wait_cycles(DELAY_CYCLES);
    let val: u8;
    asm!("inb %dx, %al" : "={al}"(val) : "{dx}"(port) :: "volatile");
    val
}

// PortLock

pub trait Port {
    const ADDR: u16;
}

// TODO: Improve this when const generics comes
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
