// Library
use core::fmt;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct StackFrame {
	rbp: u64,
	r15: u64,
	r14: u64,
	r13: u64,
	r12: u64,
	r11: u64,
	r10: u64,
	r9: u64,
	r8: u64,
	rdi: u64,
	rsi: u64,
	rdx: u64,
	rcx: u64,
	rbx: u64,
	rax: u64,
	kind: u64,
	error: u64,
	rip: u64,
	cs: u64,
	rflags: u64,
	rsp: u64,
	ss: u64,
}


impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f,
                "\
                \trip:   0x{:X}\n\
                \trsp:   0x{:X}\n\
                \tkind:  0x{:X}\n\
                \terror: 0x{:X}\n\
                \tcs:    0x{:X}\n\
                \tss:    0x{:X}",
                self.rip, self.rsp, self.kind, self.error, self.cs, self.ss
            )
        }
	}
}
