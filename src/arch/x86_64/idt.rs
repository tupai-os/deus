use core::{mem, arch::asm};
use spin::RwLock;
use bitflags::bitflags;

bitflags! {
    struct Attributes: u8 {
        const INT_GATE = 0b00001110;
        const RING0    = 0b00000000;
        const RING1    = 0b00100000;
        const RING2    = 0b01000000;
        const RING3    = 0b01100000;
        const PRESENT  = 0b10000000;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entry {
    base_lo: u16,
    selector: u16,
    _zero0: u8,
    attrib: u8,
    base_mid: u16,
    base_hi: u32,
    _zero1: u32,
}

impl Entry {
    pub const fn empty() -> Self {
        Self {
            base_lo: 0,
            selector: 0,
            _zero0: 0,
            attrib: 0,
            base_mid: 0,
            base_hi: 0,
            _zero1: 0,
        }
    }

    pub fn from_addr(addr: u64) -> Self {
        Self {
            base_lo: (addr >> 0) as u16,
            base_mid: (addr >> 16) as u16,
            base_hi: (addr >> 32) as u32,
            selector: 8, // TODO: Find the correct GDT selector
            attrib: (
                Attributes::INT_GATE |
                Attributes::RING0 |
                Attributes::PRESENT
            ).bits(),
            _zero0: 0,
            _zero1: 0,
        }
    }
}

#[repr(C, packed)]
pub struct Ptr {
    lim: u16,
    base: u64,
}

const IRQ_ENTRY_COUNT: usize = 256;

#[repr(C)]
#[repr(align(4096))]
pub struct Table {
    fault_handlers: [Entry; 32],
    irq_handlers: [Entry; IRQ_ENTRY_COUNT],
}

impl Table {
    const fn empty() -> Self {
        Self {
            fault_handlers: [Entry::empty(); 32],
            irq_handlers: [Entry::empty(); IRQ_ENTRY_COUNT],
        }
    }

    pub fn set_fault_handler(&mut self, vec: usize, entry: Entry) {
        self.fault_handlers.get_mut(vec).map(|e| *e = entry)
            .expect("Attempted to set out-of-bounds IDT fault entry");
    }

    pub fn set_irq_handler(&mut self, vec: usize, entry: Entry) {
        self.irq_handlers.get_mut(vec).map(|e| *e = entry)
            .expect("Attempted to set out-of-bounds IDT IRQ entry");
    }

    fn get_ptr(&self) -> Ptr {
        Ptr {
            lim: mem::size_of::<Self>() as u16 - 1,
            base: self as *const _ as u64,
        }
    }

    pub unsafe fn flush(&self) {
        let ptr = self.get_ptr();
        asm!("lidt [{0}]", in(reg) &ptr);
    }
}

static IDT: RwLock<Table> = RwLock::new(Table::empty());

pub fn init() {
    unsafe { IDT.read().flush(); }
}

#[allow(dead_code)]
pub fn singleton() -> &'static RwLock<Table> {
    &IDT
}
