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

Basically just https://wiki.osdev.org/RISC-V_Bare_Bones but in rust.