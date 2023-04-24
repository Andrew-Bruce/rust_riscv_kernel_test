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
*(.init);
*(.text);
} >ram AT>ram :text

PROVIDE(global_pointer = .);

.rodata : ALIGN(4K) {
*(.rodata);
} >ram AT>ram :data

.data : ALIGN(4K) {
*(.data);
} >ram AT>ram :data

.bss : ALIGN(4K) {
PROVIDE(bss_start = .);
*(.bss);
PROVIDE(bss_end = .);
} >ram AT>ram :bss

PROVIDE(stack_top = . );
PROVIDE(stack_bot = stack_top + 0x80000); /* make our stack reasonably big */

PROVIDE(memory_end = ORIGIN(ram) + LENGTH(ram));
PROVIDE(heap_start = stack_bot);
PROVIDE(heap_size = memory_end - heap_start); /* rest of ram goes to heap */

}
