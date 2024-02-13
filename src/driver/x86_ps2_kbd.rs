use spin::Mutex;
#[cfg(target_arch = "x86_64")]
use crate::{
    arch::x86_64::{port::{in8, out8}, isr::StackFrame},
    hw::x86::pic,
    startup::DriverInfo,
    ipc, iface,
    log, logln,
};

const IRQ_VEC: u8 = 1;

pub static DRIVER_INFO: DriverInfo = DriverInfo::new(
    "x86_ps2_kbd",
    init,
);

static mut TX: Option<ipc::Tx<iface::Char>> = None;

#[allow(dead_code)]
pub fn init() {
    unsafe { TX = Some(ipc::exchange().register_tx("kbd".into())) }

    // TODO
    pic::unmask(IRQ_VEC);
}

// Ports

const PORT_CMD: u16 = 0x60;
const PORT_DATA: u16 = 0x60;
const PORT_STATUS: u16 = 0x64;

// Masks

const KEY_PRESSED: u8 = 1 << 7;
const KEY_MOD: u8 = !(1 << 7);

const KEY_LSHIFT: u8 = 0x2A;
const KEY_LCTRL: u8 = 0x1D;
const KEY_LALT: u8 = 0x38;

// Scancode LUT

const SCANCODES_US: [char; 128] = [
	'!', '\x1B', '1', '2', '3', '4', '5', '6', '7', '8',	// 9
	'9', '0', '-', '=',
	'\x08', // Backspace
	'\t', // Tab
	'q', 'w', 'e', 'r',	// 19
	't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', // Enter key
	'!', // 29 - Left control
	'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', // 39
	'\'', '`',
	'!', // Left shift
	'\\', 'z', 'x', 'c', 'v', 'b', 'n', // 49
	'm', ',', '.', '/',
	'!', // Right shift
	'*',
	'!',  // Alt
	' ', // Space bar
	'!', // Caps lock
	'!', // 59 - F1 key ... >
	'!', '!',   '!',   '!',   '!',   '!',   '!',   '!',
	'!', // < ... F10
	'!', // 69 - Num lock
	'!', // Scroll Lock
	'!', // Home key
	'!', // Up Arrow
	'!', // Page Up
	'-',
	'!', // Left Arrow
	'!',
	'!', // Right Arrow
	'+',
	'!', // 79 - End key
	'!', // Down Arrow
	'!', // Page Down
	'!', // Insert Key
	'!', // Delete Key
	'!', '!', '!',
	'!', // F11 Key
	'!', // F12 Key
	// All other keys are undefined
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
	'!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!'
];

#[no_mangle]
pub extern "C" fn kbd_handler(frame: *mut StackFrame) -> *mut StackFrame {
    while unsafe { in8(PORT_STATUS) } & 1 > 0 { // Serial port is not empty
        let scancode = unsafe { in8(PORT_DATA) };
        if scancode & KEY_PRESSED == 0 {
            let c = SCANCODES_US[(scancode & 0x7F) as usize];

            match scancode & KEY_MOD {
                KEY_LSHIFT => {},
                KEY_LCTRL => {},
                KEY_LALT => {},
                _ => {},
            }

            unsafe { TX.as_ref() }
                .unwrap()
                .send(iface::Char(c as u32))
                .unwrap();
        }
    }

    pic::eoi(IRQ_VEC);
    frame
}
