global_asm!(include_str!("irq.s"));

// Local
use crate::arch::cpu::active::idt;

extern "C" {
	fn _pit_handler();
	fn _kbd_handler();
	fn _com2_handler();
	fn _com1_handler();
	fn _spurious_handler();
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
    idt_guard.set_irq_handler(0, idt::Entry::from_addr(_pit_handler as u64));
    idt_guard.set_irq_handler(1, idt::Entry::from_addr(_kbd_handler as u64));
    idt_guard.set_irq_handler(3, idt::Entry::from_addr(_com2_handler as u64));
    idt_guard.set_irq_handler(4, idt::Entry::from_addr(_com1_handler as u64));
    idt_guard.set_irq_handler(7, idt::Entry::from_addr(_spurious_handler as u64));
    unsafe { idt_guard.flush(); }
}
