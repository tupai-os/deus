// Library
use bitflags::bitflags;

// Local
use crate::arch::cpu::active::port::{Port, PortLock};

bitflags! {
    struct Icw1: u8 {
        const INIT     = 0b00010000;
        const ICW4     = 0b00000001;
        const SINGLE   = 0b00000010;
        const INTERVAL = 0b00000100;
        const LEVEL    = 0b00001000;
    }
}

bitflags! {
    struct Icw4: u8 {
        const MODE8086 = 0b00000001;
        const AUTO     = 0b00000010;
        const SLAVE    = 0b00001000;
        const MASTER   = 0b00011000;
        const NESTED   = 0b00010000;
    }
}

bitflags! {
    struct Cmd: u8 {
        const EOI = 0x20;
    }
}

// Ports

struct Pic1Cmd;
impl Port for Pic1Cmd { const ADDR: u16 = 0x20; }
static PIC1_CMD: PortLock<Pic1Cmd> = PortLock::new();

struct Pic1Data;
impl Port for Pic1Data { const ADDR: u16 = Pic1Cmd::ADDR + 1; }
static PIC1_DATA: PortLock<Pic1Data> = PortLock::new();

struct Pic2Cmd;
impl Port for Pic2Cmd { const ADDR: u16 = 0xA0; }
static PIC2_CMD: PortLock<Pic2Cmd> = PortLock::new();

struct Pic2Data;
impl Port for Pic2Data { const ADDR: u16 = Pic2Cmd::ADDR + 1; }
static PIC2_DATA: PortLock<Pic2Data> = PortLock::new();

pub fn init() {
    // Initiate configuration
    PIC1_CMD.write((Icw1::INIT | Icw1::ICW4).bits());
    PIC2_CMD.write((Icw1::INIT | Icw1::ICW4).bits());

    // Pass 3 initiation bytes to each PIC

    // 1) Remap IRQs to +32 in the interrupt vector space
    PIC1_DATA.write(32);
    PIC2_DATA.write(32 + 8);

    // 2) Give them a cascade identity
    PIC1_DATA.write(4); // Master
    PIC2_DATA.write(2); // Slave

    // 3) Tell them to operate in 8086 mode
    PIC1_DATA.write(Icw4::MODE8086.bits());
    PIC2_DATA.write(Icw4::MODE8086.bits());

    // Finally, mask all interrupts (initially)
    PIC1_DATA.write(0xFF);
    PIC2_DATA.write(0xFF);
}

pub fn unmask(vec: u8) {

    let mask = !(1 << (vec & 0b111));

    match vec {
        0...7 => PIC1_DATA.write(PIC1_DATA.read() & mask),
        8...15 => PIC2_DATA.write(PIC2_DATA.read() & mask),
        _ => panic!("Attempted to unmask invalid interrupt vector {}"),
    }
}

pub fn eoi(vec: u8) {
    if vec < 8 {
        PIC1_CMD.write(Cmd::EOI.bits());
    } else if vec < 16 {
        PIC1_CMD.write(Cmd::EOI.bits());
        PIC2_CMD.write(Cmd::EOI.bits());
    } else {
        panic!("Attempted to end interrupt for invalid vector {}");
    }
}
