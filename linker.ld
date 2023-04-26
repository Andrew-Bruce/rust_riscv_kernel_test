ENTRY(start);
OUTPUT_ARCH( "riscv" )


/* Memrory starts at 0x80000000 and has length 0x8000000, exacly 128M */
MEMORY {
ram (wxa!ri) : ORIGIN = 0x80000000, LENGTH = 0x8000000
}


PHDRS
{
text PT_LOAD;
data PT_LOAD;
bss PT_LOAD;
}

SECTIONS {
/* Include entry point at start of binary */
.text : ALIGN(4K) {
      PROVIDE(text_start = .);
      *(.init);
      *(.text .text.*)
      PROVIDE(text_end = .);
} >ram AT>ram :text

PROVIDE(global_pointer = .);

.rodata : ALIGN(4K) {
	PROVIDE(rodata_start = .);
	*(.rodata .rodata.*)
	PROVIDE(rodata_end = .);
} >ram AT>ram :text

.data : ALIGN(4K) {
      PROVIDE(data_start = .);
      *(.sdata .sdata.*) *(.data .data.*)
      PROVIDE(data_end = .);
} >ram AT>ram :data

.bss : ALIGN(4K) {
     PROVIDE(bss_start = .);
     *(.sbss .sbss.*) *(.bss .bss.*)
     PROVIDE(bss_end = .);
} >ram AT>ram :bss


PROVIDE(memory_start = ORIGIN(ram));
PROVIDE(memory_end = ORIGIN(ram) + LENGTH(ram));

. = ALIGN(4K);
PROVIDE(stack_bot = . );
PROVIDE(stack_top = stack_bot + 0x80000); /* make our stack reasonably big */

. = ALIGN(4K);
PROVIDE(heap_start = stack_top);
PROVIDE(heap_end = memory_end); /* rest of ram goes to heap */
PROVIDE(heap_size = heap_end - heap_start); /* rest of ram goes to heap */

}
