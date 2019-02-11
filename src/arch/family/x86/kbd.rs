// Library
use bitflags::bitflags;
use arraydeque::ArrayDeque;
use spin::Mutex;

// Local
use super::pic;
use crate::arch::cpu::active::{
    idt,
    port::{Port, PortLock},
};
use crate::log;

const IRQ_VEC: u8 = 1;

// Ports

struct Cmd;
impl Port for Cmd { const ADDR: u16 = 0x43; }
static CMD: PortLock<Cmd> = PortLock::new();

struct Data;
impl Port for Data { const ADDR: u16 = 0x40; }
static DATA: PortLock<Data> = PortLock::new();

#[no_mangle]
extern "C" fn kbd_handler(frame: *const ()) -> *const () {
    log!("!");

    pic::eoi(IRQ_VEC);
    frame
}

lazy_static! {
    static ref CHAR_BUF: Mutex<ArrayDeque<[char; 1024]>> = Mutex::new(ArrayDeque::new());
}

pub fn init() {
    // TODO
    pic::unmask(IRQ_VEC);
}
