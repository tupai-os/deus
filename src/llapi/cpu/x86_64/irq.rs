// Local
use super::idt;

#[no_mangle]
extern "C" fn pit_handler(frame: *const ()) -> *const () {
    panic!("PIT interrupt!");
    frame
}

#[no_mangle]
extern "C" fn kbd_handler(frame: *const ()) -> *const () {
    panic!("Keyboard interrupt!");
    frame
}

#[no_mangle]
extern "C" fn com2_handler(frame: *const ()) -> *const () {
    panic!("COM2 interrupt!");
    frame
}

#[no_mangle]
extern "C" fn com1_handler(frame: *const ()) -> *const () {
    panic!("COM1 interrupt!");
    frame
}

#[no_mangle]
extern "C" fn spurious_handler(frame: *const ()) -> *const () {
    frame
}

pub fn init() {
    let mut idt_guard = idt::singleton().write();
    idt_guard.set_irq_handler(0, idt::Entry::from_addr(pit_handler as u64));
    unsafe { idt_guard.flush(); }
}
