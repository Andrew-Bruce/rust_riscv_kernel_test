#CFLAGS = -Wall -Wextra -g -ffreestanding -nostdlib
ASFLAGS = -g 
LDFLAGS = -Tlinker.ld -nostdlib -L./target/riscv64gc-unknown-none-elf/debug -g
LDLIBS = -lchad_os

#CC = riscv64-unknown-linux-gnu-gcc
AS = riscv64-unknown-linux-gnu-as
LD = riscv64-unknown-linux-gnu-ld

RUN = qemu-system-riscv64 -machine virt -bios none -kernel kernel.elf -serial mon:stdio

OBJS = entry.o symbols.o

.PHONY: clean run debug kernel.elf


kernel.elf: $(OBJS)
	cargo build
	$(LD) $(ASFLAGS) $^ $(LDFLAGS) $(LDLIBS) -o $@

entry.o: entry.S
	$(AS) $(ASFLAGS) -c entry.S -o $(@)

symbols.o: symbols.S
	$(AS) $(ASFLAGS) -c symbols.S -o $(@)

run: kernel.elf
	$(RUN)

debug: kernel.elf
	$(RUN) -gdb tcp::1234 -S

clean:
	cargo clean
	$(RM) kernel.elf $(OBJS)
