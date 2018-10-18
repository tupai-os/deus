// Library
use core::mem;
use spin::RwLock;

#[allow(dead_code)]
mod attrib {
    pub const INT_GATE: u8 = 0b00000000;
    pub const DPL0: u8     = 0b00000000;
    pub const DPL1: u8     = 0b00100000;
    pub const DPL2: u8     = 0b01000000;
    pub const DPL3: u8     = 0b01100000;
    pub const PRESENT: u8  = 0b10000000;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Entry {
    base_lo: u16,
    selector: u16,
    _zero0: u8,
    attrib: u8,
    base_mid: u16,
    base_hi: u32,
    _zero1: u32,
}

impl Entry {
    const fn empty() -> Self {
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

    fn from_addr(addr: u64) -> Self {
        Self {
            base_lo: (addr >> 0) as u16,
            base_mid: (addr >> 16) as u16,
            base_hi: (addr >> 32) as u32,
            selector: 0, // TODO: Find the correct GDT selector
            attrib: attrib::INT_GATE | attrib::DPL0 | attrib::PRESENT,
            _zero0: 0,
            _zero1: 0,
        }
    }
}

#[repr(C, packed)]
struct Ptr {
    lim: u16,
    base: u64,
}

#[allow(dead_code)]
#[repr(usize)]
pub enum Exception {
	DivZero          = 0,
	Debug            = 1,
	NMI              = 2,
	Breakpoint       = 3,
	Overflow         = 4,
	BoundRange       = 5,
	InvalidOpcode    = 6,
	NoFPU            = 7,
	DoubleFault      = 8,
	FPUSegFault      = 9,
	InvalidTSS       = 10,
	SegNotPresent    = 11,
	StackSegFault    = 12,
	GPF              = 13,
	PageFault        = 14,
	// <Reserved>
	FPUException     = 16,
	AlignmentCheck   = 17,
	MachineCheck     = 18,
	SIMDError        = 19,
	VirtError        = 20,
	// <Reserved>
	SecurityError    = 30,
	// <Reserved>
}

const IRQ_ENTRY_COUNT: usize = 256;

#[repr(C)]
#[repr(align(16))]
pub struct Table {
    exception_handlers: [Entry; 32],
    irq_handlers: [Entry; IRQ_ENTRY_COUNT],
}

impl Table {
    const fn empty() -> Self {
        Self {
            exception_handlers: [
                Entry::empty(), // 0
                Entry::empty(), // 1
                Entry::empty(), // 2
                Entry::empty(), // 3
                Entry::empty(), // 4
                Entry::empty(), // 5
                Entry::empty(), // 6
                Entry::empty(), // 7
                Entry::empty(), // 8
                Entry::empty(), // 9
                Entry::empty(), // 10
                Entry::empty(), // 11
                Entry::empty(), // 12
                Entry::empty(), // 13
                Entry::empty(), // 14
                Entry::empty(), // 15
                Entry::empty(), // 16
                Entry::empty(), // 17
                Entry::empty(), // 18
                Entry::empty(), // 19
                Entry::empty(), // 20
                Entry::empty(), // 21
                Entry::empty(), // 22
                Entry::empty(), // 23
                Entry::empty(), // 24
                Entry::empty(), // 25
                Entry::empty(), // 26
                Entry::empty(), // 27
                Entry::empty(), // 28
                Entry::empty(), // 29
                Entry::empty(), // 30
                Entry::empty(), // 31
            ],
            irq_handlers: [Entry::empty(); IRQ_ENTRY_COUNT],
        }
    }

    fn set_exception_handler(&mut self, vec: usize, entry: Entry) {
        self.exception_handlers.get_mut(vec).map(|e| *e = entry)
            .expect("Attempted to set out-of-bounds IDT exception entry");
    }

    fn set_irq_handler(&mut self, vec: usize, entry: Entry) {
        self.irq_handlers.get_mut(vec).map(|e| *e = entry)
            .expect("Attempted to set out-of-bounds IDT IRQ entry");
    }

    fn get_ptr(&self) -> Ptr {
        Ptr {
            lim: mem::size_of::<Self>() as u16 - 1,
            base: self as *const _ as u64,
        }
    }

    unsafe fn flush(&self) {
        let ptr = self.get_ptr();
        asm!("lidt ($0)" :: "r"(&ptr) : "memory");
    }
}

static IDT: RwLock<Table> = RwLock::new(Table::empty());

pub fn init() {
    unsafe { IDT.read().flush(); }
}

pub fn singleton() -> &'static RwLock<Table> {
    &IDT
}
