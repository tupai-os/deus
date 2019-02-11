// Library
use bitflags::bitflags;

// Local
use super::pic;
use crate::arch::cpu::active::{
    idt,
    port::{Port, PortLock},
};
use crate::log;

bitflags! {
    struct Cfg: u8 {
        const SQUARE_WAVE = 0b00000110;
        const LO_HI       = 0b00110000;
    }
}

const IRQ_VEC: u8 = 0;

// Ports

struct Cmd;
impl Port for Cmd { const ADDR: u16 = 0x43; }
static CMD: PortLock<Cmd> = PortLock::new();

struct Data;
impl Port for Data { const ADDR: u16 = 0x40; }
static DATA: PortLock<Data> = PortLock::new();

#[no_mangle]
extern "C" fn pit_handler(frame: *const ()) -> *const () {
    log!("!");

    pic::eoi(IRQ_VEC);
    frame
}

pub fn init() {
    set_rate(100);
    pic::unmask(IRQ_VEC);
}

fn set_rate(hz: u32) {
    // Begin PIT configuration
    CMD.write((Cfg::SQUARE_WAVE | Cfg::LO_HI).bits());

    // Calculate divisor from hertz
    let div = 1193180 / hz;

    // Write lo and hi bytes
    DATA.write((div >> 0) as u8);
    DATA.write((div >> 8) as u8);
}
