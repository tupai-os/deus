#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{idt, isr::StackFrame};

use core::arch::global_asm;

//global_asm!(include_str!("irq.s"));

/*
extern "C" {
	fn _pit_handler();
	fn _kbd_handler();
	fn _com2_handler();
	fn _com1_handler();
	fn _spurious_handler();
}
*/

#[no_mangle]
extern "C" fn com2_handler(frame: *mut StackFrame) -> *mut StackFrame {
    panic!("COM2 interrupt!");
    frame
}

#[no_mangle]
extern "C" fn com1_handler(frame: *mut StackFrame) -> *mut StackFrame {
    panic!("COM1 interrupt!");
    frame
}

#[no_mangle]
extern "C" fn spurious_handler(frame: *mut StackFrame) -> *mut StackFrame {
    crate::logln!("Spurious");
    frame
}

/*
pub fn init() {
    let mut idt_guard = idt::singleton().write();
    idt_guard.set_irq_handler(0, idt::Entry::from_addr(_pit_handler as u64));
    idt_guard.set_irq_handler(1, idt::Entry::from_addr(_kbd_handler as u64));
    idt_guard.set_irq_handler(3, idt::Entry::from_addr(_com2_handler as u64));
    idt_guard.set_irq_handler(4, idt::Entry::from_addr(_com1_handler as u64));
    idt_guard.set_irq_handler(7, idt::Entry::from_addr(_spurious_handler as u64));
    unsafe { idt_guard.flush(); }
}
*/

macro_rules! handlers {
    ($($irq:literal => $handler:path),* $(,)?) => {
        pub fn init() {
            let mut idt = idt::singleton().write();
            $({
                const _: extern "C" fn(*mut StackFrame) -> *mut StackFrame = $handler;
                #[naked]
                unsafe fn isr() {
                    core::arch::asm!(
                        // Push registers
                        "push 0", // TODO: Don't push $0 as a fake error code
                        stringify!(push $irq),
                        "push rax",
                        "push rbx",
                        "push rcx",
                        "push rdx",
                        "push rsi",
                        "push rdi",
                        "push r8",
                        "push r9",
                        "push r10",
                        "push r11",
                        "push r12",
                        "push r13",
                        "push r14",
                        "push r15",
                        "push rbp",
                        "cld",
                        
                        // Call handler with old stack frame
                        "mov rdi, rsp",
                        "call {0}", 
                        "mov rsp, rax", // Swap out new stack frame
                        
                        // Pop registers
                        "pop rbp",
                        "pop r15",
                        "pop r14",
                        "pop r13",
                        "pop r12",
                        "pop r11",
                        "pop r10",
                        "pop r9",
                        "pop r8",
                        "pop rdi",
                        "pop rsi",
                        "pop rdx",
                        "pop rcx",
                        "pop rbx",
                        "pop rax",
                        
                        // Remove error and ID from stack
                        "add rsp, 16",
                        
                        // Return from interrupt
                        "iretq",
                        sym $handler,
                        options(noreturn),
                    );
                }
                idt.set_irq_handler($irq, idt::Entry::from_addr(isr as u64));
            })*
            unsafe { idt.flush(); }
        }
    };
}

handlers! {
    0 => super::pit::pit_handler,
    1 => crate::driver::x86_ps2_kbd::kbd_handler,
    3 => com2_handler,
    4 => com1_handler,
    7 => spurious_handler,
    // TODO: others
}
