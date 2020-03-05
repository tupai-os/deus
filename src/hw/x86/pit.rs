use bitflags::bitflags;
use super::pic;
#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::port::{Port, PortLock};
use crate::{
    log,
    hal::StackFrame,
    sched::preempt,
};

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
extern "C" fn pit_handler(frame: *mut StackFrame) -> *mut StackFrame {
    let frame = preempt(frame);

    pic::eoi(IRQ_VEC);

    frame
}

pub fn init() {
    set_rate(1000);
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
