// Local
use super::super::super::super::{log, logln}; // TODO: Why is this needed?!
use super::{idt, isr::StackFrame};

extern "C" {
	fn _exception_handler0();
	fn _exception_handler1();
	fn _exception_handler2();
	fn _exception_handler3();
	fn _exception_handler4();
	fn _exception_handler5();
	fn _exception_handler6();
	fn _exception_handler7();
	fn _exception_handler8();
	fn _exception_handler9();
	fn _exception_handler10();
	fn _exception_handler11();
	fn _exception_handler12();
	fn _exception_handler13();
	fn _exception_handler14();
	// <Reserved>
	fn _exception_handler16();
	fn _exception_handler17();
	fn _exception_handler18();
	fn _exception_handler19();
	fn _exception_handler20();
	// <Reserved>
	fn _exception_handler30();
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
    panic!("Division by zero exception!");
    frame
}

#[no_mangle]
extern "C" fn debug_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("Debug exception!");
    frame
}

#[no_mangle]
extern "C" fn gpf_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("General protection fault exception!");
    frame
}

#[no_mangle]
extern "C" fn pagefault_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("Page fault exception!");
    frame
}

#[no_mangle]
extern "C" fn misc_handler(frame: *mut StackFrame) -> *mut StackFrame {
    logln!("{}", unsafe { &*frame });
    panic!("Unimplemented handler exception!");
    frame
}

pub fn init() {
    let mut idt_guard = idt::singleton().write();
    idt_guard.set_exception_handler(0, idt::Entry::from_addr(_exception_handler0 as u64));
    idt_guard.set_exception_handler(1, idt::Entry::from_addr(_exception_handler1 as u64));
    idt_guard.set_exception_handler(2, idt::Entry::from_addr(_exception_handler2 as u64));
    idt_guard.set_exception_handler(3, idt::Entry::from_addr(_exception_handler3 as u64));
    idt_guard.set_exception_handler(4, idt::Entry::from_addr(_exception_handler4 as u64));
    idt_guard.set_exception_handler(5, idt::Entry::from_addr(_exception_handler5 as u64));
    idt_guard.set_exception_handler(6, idt::Entry::from_addr(_exception_handler6 as u64));
    idt_guard.set_exception_handler(7, idt::Entry::from_addr(_exception_handler7 as u64));
    idt_guard.set_exception_handler(8, idt::Entry::from_addr(_exception_handler8 as u64));
    idt_guard.set_exception_handler(9, idt::Entry::from_addr(_exception_handler9 as u64));
    idt_guard.set_exception_handler(10, idt::Entry::from_addr(_exception_handler10 as u64));
    idt_guard.set_exception_handler(11, idt::Entry::from_addr(_exception_handler11 as u64));
    idt_guard.set_exception_handler(12, idt::Entry::from_addr(_exception_handler12 as u64));
    idt_guard.set_exception_handler(13, idt::Entry::from_addr(_exception_handler13 as u64));
    idt_guard.set_exception_handler(14, idt::Entry::from_addr(_exception_handler14 as u64));
    // Reserved
    idt_guard.set_exception_handler(16, idt::Entry::from_addr(_exception_handler16 as u64));
    idt_guard.set_exception_handler(17, idt::Entry::from_addr(_exception_handler17 as u64));
    idt_guard.set_exception_handler(18, idt::Entry::from_addr(_exception_handler18 as u64));
    idt_guard.set_exception_handler(19, idt::Entry::from_addr(_exception_handler19 as u64));
    idt_guard.set_exception_handler(20, idt::Entry::from_addr(_exception_handler20 as u64));
    // Reserved
    idt_guard.set_exception_handler(30, idt::Entry::from_addr(_exception_handler30 as u64));
    // Reserved
    unsafe { idt_guard.flush(); }
}
