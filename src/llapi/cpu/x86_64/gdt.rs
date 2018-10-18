// Library
use core::mem;
use spin::RwLock;
use bitflags::bitflags;

bitflags! {
    struct Access: u8 {
        const ReadWrite  = 0b00000010;
        const Execute    = 0b00001000;
        const Present    = 0b10000000;
        const One        = 0b00010000;
        const Ring0      = 0b00000000;
        const Ring1      = 0b00100000;
        const Ring2      = 0b01000000;
        const Ring3      = 0b01100000;
        const Conforming = 0b00000100;
    }
}

bitflags! {
    struct Gran: u8 {
        const Page = 0b00001000;
        const Long = 0b00000010;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entry {
    lim_lo: u16,
    base_lo: u16,
    base_mid: u8,
    access: u8,
    gran: u8,
    base_hi: u8,
}

impl Entry {
    const fn empty() -> Self {
        Self {
            lim_lo: 0,
            base_lo: 0,
            base_mid: 0,
            access: 0,
            gran: 0,
            base_hi: 0,
        }
    }

    fn new(base: u64, lim: usize, access: Access) -> Self {
        Self {
            base_lo: (base >> 0) as u16,
            base_mid: (base >> 16) as u8,
            base_hi: (base >> 24) as u8,

            lim_lo: lim as u16,

            access: access.bits(),
            gran:
                ((Gran::Page | Gran::Long).bits() << 4) |
                ((lim >> 16) as u8 & 0x0F),
        }
    }
}

#[repr(C, packed)]
pub struct Ptr {
    lim: u16,
    base: u64,
}

const SEG_ENTRY_COUNT: usize = 5;

#[repr(C)]
#[repr(align(16))]
pub struct Table {
    entries: [Entry; SEG_ENTRY_COUNT],
}

impl Table {
    const fn empty() -> Self {
        Self { entries: [Entry::empty(); SEG_ENTRY_COUNT] }
    }

    fn default() -> Self {
        let code_access =
            Access::ReadWrite |
            Access::Execute |
            Access::One |
            Access::Conforming |
            Access::Present;

        let data_access =
            Access::ReadWrite |
            Access::One |
            Access::Present;

        Self {
            entries: [
                Entry::empty(),
                Entry::new(0x0, 0xFFFFF, code_access | Access::Ring0),
                Entry::new(0x0, 0xFFFFF, data_access | Access::Ring0),
                Entry::new(0x0, 0xFFFFF, code_access | Access::Ring3),
                Entry::new(0x0, 0xFFFFF, data_access | Access::Ring3),
            ],
        }
    }

    fn set_entry(&mut self, vec: usize, entry: Entry) {
        self.entries.get_mut(vec).map(|e| *e = entry)
            .expect("Attempted to set out-of-bounds GDT segment entry");
    }

    fn get_ptr(&self) -> Ptr {
        Ptr {
            lim: mem::size_of::<Self>() as u16 - 1,
            base: self as *const _ as u64,
        }
    }

    pub unsafe fn flush(&self) {
        let ptr = self.get_ptr();
        asm!("
            lgdt ($0);
            mov $$0x10, %ax;
            mov %ax, %ds;
            mov %ax, %fs;
            mov %ax, %es;
            mov %ax, %gs;
            mov %ax, %ss;
            " :: "r"(&ptr) : "memory"
        );
    }
}

static GDT: RwLock<Table> = RwLock::new(Table::empty());

pub fn init() {
    *GDT.write() = Table::default();
    unsafe { GDT.read().flush(); }
}

pub fn singleton() -> &'static RwLock<Table> {
    &GDT
}
