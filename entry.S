	.section .init
	
	.option norvc
	
	.type start, @function
	.global start
start:
	.cfi_startproc

	/* if core not cpu0 skip this and wait for interrupt */
	csrr t0, mhartid
	bnez t0, loop_forever

	/* Reset satp */
	csrw satp, zero

	.option push
	.option norelax /* dont optimize, sometimes assumes gp is already initialized */
	la gp, global_pointer
	.option pop

	/* Clear the BSS section */
	la t0, bss_start
	la t1, bss_end
bss_clear:
	sd zero, (t0)
	addi t0, t0, 8
	bleu t0, t1, bss_clear
	
	/* Setup stack */
	la sp, stack_top

	/* set exeption counter */
	la t0, kmain
	csrw mepc, t0

	la ra, loop_forever

	/* Jump to kernel! */
	tail kmain
	
	.cfi_endproc

loop_forever:
	wfi
	j loop_forever

	.end
