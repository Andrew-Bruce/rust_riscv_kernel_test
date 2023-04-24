	/* some symbols from the linker rust will need */
	/* mostly info about the memory layout and heap size */

	.section .rodata

	.global HEAP_START
HEAP_START: .dword heap_start
	.global HEAP_SIZE 
HEAP_SIZE: .dword heap_size

	.global STACK_TOP
STACK_TOP: .dword stack_top
	.global STACK_BOT
STACK_BOT: .dword stack_bot

	.global BSS_START
BSS_START: .dword bss_start
	.global BSS_END 
BSS_END: .dword bss_end

	.global MEMORY_START
MEMORY_START: .dword memory_start
	.global MEMORY_END
MEMORY_END: .dword memory_end
