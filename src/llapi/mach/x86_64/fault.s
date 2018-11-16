.set FAULT_DUMMY_ERROR, 0xFFFFFFFFFFFFFFFF

.code64
.section .text
	.macro PUSH_REGS
		push %rax
		push %rbx
		push %rcx
		push %rdx
		push %rsi
		push %rdi
		push %r8
		push %r9
		push %r10
		push %r11
		push %r12
		push %r13
		push %r14
		push %r15
		push %rbp
		cld
	.endm

	.macro POP_REGS
		pop %rbp
		pop %r15
		pop %r14
		pop %r13
		pop %r12
		pop %r11
		pop %r10
		pop %r9
		pop %r8
		pop %rdi
		pop %rsi
		pop %rdx
		pop %rcx
		pop %rbx
		pop %rax
	.endm

	.macro ERROR_FAULT n, name
		.align 8
		.global _fault_handler\n\()
		_fault_handler\n\():
			push $\n\() // Push fault ID
			PUSH_REGS
			mov %rsp, %rdi // Pass stack frame
			.extern \name\()_handler
			call \name\()_handler
			mov %rax, %rsp // Swap out new stack frame
			POP_REGS
			add $16, %rsp // Remove error and ID from stack
			iretq
	.endm

	.macro DUMMY_FAULT n, name
		.align 8
		.global _fault_handler\n\()
		_fault_handler\n\():
			push $FAULT_DUMMY_ERROR // Dummy error
			push $\n\() // Push fault ID
			PUSH_REGS
			mov %rsp, %rdi // Pass stack frame
			.extern \name\()_handler
			call \name\()_handler
			mov %rax, %rsp // Swap out new stack frame
			POP_REGS
			add $16, %rsp // Remove error and ID from stack
			iretq
	.endm

	DUMMY_FAULT 0 divzero
	DUMMY_FAULT 1 debug
	DUMMY_FAULT 2 misc
	DUMMY_FAULT 3 misc
	DUMMY_FAULT 4 misc
	DUMMY_FAULT 5 misc
	DUMMY_FAULT 6 misc
	DUMMY_FAULT 7 misc
	ERROR_FAULT 8 misc
	DUMMY_FAULT 9 misc
	ERROR_FAULT 10 misc
	ERROR_FAULT 11 misc
	ERROR_FAULT 12 misc
	ERROR_FAULT 13 gpf
	ERROR_FAULT 14 pagefault
	DUMMY_FAULT 16 misc
	ERROR_FAULT 17 misc
	DUMMY_FAULT 18 misc
	DUMMY_FAULT 19 misc
	DUMMY_FAULT 20 misc
	ERROR_FAULT 30 misc
