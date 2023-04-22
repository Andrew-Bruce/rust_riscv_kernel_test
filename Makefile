


CC = riscv64-unknown-linux-gnu-gcc
AS = riscv64-unknown-linux-gnu-as
LD = riscv64-unknown-linux-gnu-ld


#riscv64-elf-gcc -Wall -Wextra -c -mcmodel=medany kernel.c -o kernel.o -ffreestanding
#riscv64-elf-as -c entry.S -o entry.o
#riscv64-elf-ld -T linker.ld -lgcc -nostdlib kernel.o entry.o -o kernel.elf

RUN = qemu-system-riscv64 -machine virt -bios none -kernel kernel.elf -serial mon:stdio

.PHONY: clean kernel.elf

kernel.elf:
	cargo build
	$(CC) -Wall -Wextra -g -Tlinker.ld -o kernel.elf entry.S -L./target/riscv64gc-unknown-none-elf/debug -lchad_os -ffreestanding -nostdlib

run: kernel.elf
	$(RUN)

debug: kernel.elf
	$(RUN) -gdb tcp::1234 -S



clean:
	cargo clean
	$(RM) kernel.elf kernel.o entry.o
