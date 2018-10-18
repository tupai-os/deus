// Library
use bitflags::bitflags;

// Local
use super::port::{Port, PortLock};

bitflags! {
    struct Icw1: u8 {
        const Init     = 0b00010000;
        const Icw4     = 0b00000001;
        const Single   = 0b00000010;
        const Interval = 0b00000100;
        const Level    = 0b00001000;
    }
}

bitflags! {
    struct Icw4: u8 {
        const Mode8086 = 0b00000001;
        const Auto     = 0b00000010;
        const Slave    = 0b00001000;
        const Master   = 0b00011000;
        const Nested   = 0b00010000;
    }
}

// Ports

struct Pic1Cmd;
impl Port for Pic1Cmd { const ADDR: u16 = 0x20; }
static PIC1_CMD: PortLock<Pic1Cmd> = PortLock::new();

struct Pic1Data;
impl Port for Pic1Data { const ADDR: u16 = 0x20; }
static PIC1_DATA: PortLock<Pic1Data> = PortLock::new();

struct Pic2Cmd;
impl Port for Pic2Cmd { const ADDR: u16 = Pic1Cmd::ADDR + 1; }
static PIC2_CMD: PortLock<Pic2Cmd> = PortLock::new();

struct Pic2Data;
impl Port for Pic2Data { const ADDR: u16 = Pic2Cmd::ADDR + 1; }
static PIC2_DATA: PortLock<Pic2Data> = PortLock::new();

pub fn init() {
    // Initiate configuration
    PIC1_CMD.write((Icw1::Init | Icw1::Icw4).bits());
    PIC2_CMD.write((Icw1::Init | Icw1::Icw4).bits());

    // Pass 3 initiation bytes to each PIC

    // 1) Remap IRQs to +32 in the interrupt vector space
    PIC1_DATA.write(32);
    PIC2_DATA.write(32 + 8);

    // 2) Give them a cascade identity
    PIC1_DATA.write(4); // Master
    PIC2_DATA.write(2); // Slave

    // 3) Tell them to operate in 8086 mode
    PIC1_DATA.write(Icw4::Mode8086.bits());
    PIC2_DATA.write(Icw4::Mode8086.bits());

    // Finally, mask all interrupts (initially)
    PIC1_DATA.write(0xFF);
    PIC2_DATA.write(0xFF);
}
