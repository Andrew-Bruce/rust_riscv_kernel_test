# Rust kernel for RISCV

## Build

``make clean``

``make``

## Run

``make run``

## Debugging

``make debug``

Then in run gdb and connect to qemu with

``riscv64-unknown-linux-gnu-gdb kernel.elf``

``(gdb) target remote localhost:1234``

See https://qemu-project.gitlab.io/qemu/system/gdb.html for more.

## stuff

Mostly made from https://wiki.osdev.org/RISC-V_Bare_Bones but in rust.

Figured out some stuff from https://wiki.osdev.org/RISC-V_Meaty_Skeleton_with_QEMU_virt_board

Assembly entry copied a lot from https://github.com/sgmarz/osblog/blob/master/risc_v/src/asm/boot.S

