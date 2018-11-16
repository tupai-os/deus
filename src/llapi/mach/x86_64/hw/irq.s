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

	.macro INTERRUPT n, name
		.align 8
		.global _\name\()_handler
		_\name\()_handler:
			push $FAULT_DUMMY_ERROR // Dummy error
			push $\n\() // Push interrupt vector
			PUSH_REGS
			mov %rsp, %rdi // Pass stack frame
			.extern \name\()_handler
			call \name\()_handler
			mov %rax, %rsp // Swap out new stack frame
			POP_REGS
			add $16, %rsp // Remove error and ID from stack
			iretq
	.endm

	INTERRUPT 0, pit
	INTERRUPT 1, kbd
	INTERRUPT 3, com2
	INTERRUPT 4, com1
	INTERRUPT 7, spurious
