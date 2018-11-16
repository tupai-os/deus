global_asm!(include_str!("irq.s"));

// Local
use super::super::idt;

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
    idt_guard.set_irq_handler(1, idt::Entry::from_addr(kbd_handler as u64));
    idt_guard.set_irq_handler(3, idt::Entry::from_addr(com2_handler as u64));
    idt_guard.set_irq_handler(4, idt::Entry::from_addr(com1_handler as u64));
    idt_guard.set_irq_handler(7, idt::Entry::from_addr(spurious_handler as u64));
    unsafe { idt_guard.flush(); }
}
