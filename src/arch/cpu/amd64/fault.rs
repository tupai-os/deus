global_asm!(include_str!("fault.s"));

// Local
use super::super::super::super::{log, logln}; // TODO: Why is this needed?!
use super::{idt, isr::StackFrame};

extern "C" {
	fn _fault_handler0();
	fn _fault_handler1();
	fn _fault_handler2();
	fn _fault_handler3();
	fn _fault_handler4();
	fn _fault_handler5();
	fn _fault_handler6();
	fn _fault_handler7();
	fn _fault_handler8();
	fn _fault_handler9();
	fn _fault_handler10();
	fn _fault_handler11();
	fn _fault_handler12();
	fn _fault_handler13();
	fn _fault_handler14();
	// <Reserved>
	fn _fault_handler16();
	fn _fault_handler17();
	fn _fault_handler18();
	fn _fault_handler19();
	fn _fault_handler20();
	// <Reserved>
	fn _fault_handler30();
    // Reserved
}

#[repr(usize)]
pub enum Exception {
	DivZero          = 0,
	Debug            = 1,
	NMI              = 2,
	Breakpoint       = 3,
	Overflow         = 4,
	BoundRange       = 5,
	InvalidOpcode    = 6,
	NoFPU            = 7,
	DoubleFault      = 8,
	FPUSegFault      = 9,
	InvalidTSS       = 10,
	SegNotPresent    = 11,
	StackSegFault    = 12,
	GPF              = 13,
	PageFault        = 14,
	// <Reserved>
	FPUException     = 16,
	AlignmentCheck   = 17,
	MachineCheck     = 18,
	SIMDError        = 19,
	VirtError        = 20,
	// <Reserved>
	SecurityError    = 30,
	// <Reserved>
}

#[no_mangle]
extern "C" fn divzero_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("Division by zero fault!");
    frame
}

#[no_mangle]
extern "C" fn debug_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("Debug fault!");
    frame
}

#[no_mangle]
extern "C" fn gpf_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("General protection fault fault!");
    frame
}

#[no_mangle]
extern "C" fn pagefault_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("Page fault fault!");
    frame
}

#[no_mangle]
extern "C" fn misc_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("Unimplemented handler fault!");
    frame
}

pub fn init() {
    let mut idt_guard = idt::singleton().write();
    idt_guard.set_fault_handler(0, idt::Entry::from_addr(_fault_handler0 as u64));
    idt_guard.set_fault_handler(1, idt::Entry::from_addr(_fault_handler1 as u64));
    idt_guard.set_fault_handler(2, idt::Entry::from_addr(_fault_handler2 as u64));
    idt_guard.set_fault_handler(3, idt::Entry::from_addr(_fault_handler3 as u64));
    idt_guard.set_fault_handler(4, idt::Entry::from_addr(_fault_handler4 as u64));
    idt_guard.set_fault_handler(5, idt::Entry::from_addr(_fault_handler5 as u64));
    idt_guard.set_fault_handler(6, idt::Entry::from_addr(_fault_handler6 as u64));
    idt_guard.set_fault_handler(7, idt::Entry::from_addr(_fault_handler7 as u64));
    idt_guard.set_fault_handler(8, idt::Entry::from_addr(_fault_handler8 as u64));
    idt_guard.set_fault_handler(9, idt::Entry::from_addr(_fault_handler9 as u64));
    idt_guard.set_fault_handler(10, idt::Entry::from_addr(_fault_handler10 as u64));
    idt_guard.set_fault_handler(11, idt::Entry::from_addr(_fault_handler11 as u64));
    idt_guard.set_fault_handler(12, idt::Entry::from_addr(_fault_handler12 as u64));
    idt_guard.set_fault_handler(13, idt::Entry::from_addr(_fault_handler13 as u64));
    idt_guard.set_fault_handler(14, idt::Entry::from_addr(_fault_handler14 as u64));
    // Reserved
    idt_guard.set_fault_handler(16, idt::Entry::from_addr(_fault_handler16 as u64));
    idt_guard.set_fault_handler(17, idt::Entry::from_addr(_fault_handler17 as u64));
    idt_guard.set_fault_handler(18, idt::Entry::from_addr(_fault_handler18 as u64));
    idt_guard.set_fault_handler(19, idt::Entry::from_addr(_fault_handler19 as u64));
    idt_guard.set_fault_handler(20, idt::Entry::from_addr(_fault_handler20 as u64));
    // Reserved
    idt_guard.set_fault_handler(30, idt::Entry::from_addr(_fault_handler30 as u64));
    // Reserved
    unsafe { idt_guard.flush(); }
}
