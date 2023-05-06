use crate::println;
use core::arch::asm;

fn read_misa() -> Option<usize> {
    let mut val: usize;
    unsafe {
        asm!("csrr {}, misa", out(reg) val);
    }
    if val == 0 {
        return None;
    }
    let width_bits = core::mem::size_of::<usize>() * 8;
    let mxl = val >> (width_bits - 2);
    println!("MXL = {}", mxl);
    let xlen: Option<usize> = match mxl {
        1 => Some(32),
        2 => Some(64),
        3 => Some(128),
        _ => None,
    };

    if let Some(xlen) = xlen {
        let zeros: usize = (val >> 26) & !(0b11 << ((xlen - 26) - 2));
        if zeros != 0 {
            println!("zeros of misa register isn't zero");
            println!("{:b}", zeros);
            println!("ignoring for now");
        }
    } else {
        println!("invalid MXL value of {}, should be 1, 2, or 3", mxl);
    }

    let extentions: usize = val & ((1 << 26) - 1);
    println!("misa extentions = {:#028b}", extentions);
    for i in 0..26 {
        let letter: char = (b'A' + i) as char;
        let extention_enabled: bool = extentions & (1 << i) != 0;
        if !extention_enabled {
            match letter {
                'I' => println!("base ISA not enabled?? how is this even running"),
                'U' => panic!("cpu does not support user mode??"),
                'S' => panic!("cpu does not support supervisor mode??"),
                _ => (),
            }
        }
        if extention_enabled {
            match letter {
                'A' => println!("ENABLED: Atomic extension"),
                'B' => panic!("ENABLED: Tentatively reserved for Bit-Manipulation extension"),
                'C' => println!("ENABLED: Compressed extension"),
                'D' => println!("ENABLED: Double-precision floating-point extension"),
                'E' => println!("ENABLED: RV32E base ISA"),
                'F' => println!("ENABLED: Single-precision floating-point extension"),
                'H' => println!("ENABLED: Hypervisor extension"),
                'G' => panic!("ENABLED: Reserved"),
                'I' => println!("ENABLED: RV32I/64I/128I base ISA"),
                'J' => println!(
                    "ENABLED: Tentatively reserved for Dynamically Translated Languages extension"
                ),
                'K' => panic!("ENABLED: Reserved"),
                'L' => panic!("ENABLED: Reserved"),
                'M' => println!("ENABLED: Integer Multiply/Divide extension"),
                'N' => panic!("ENABLED: Tentatively reserved for User-Level Interrupts extension"),
                'O' => panic!("ENABLED: Reserved"),
                'P' => panic!("ENABLED: Tentatively reserved for Packed-SIMD extension"),
                'Q' => println!("ENABLED: Quad-precision floating-point extension"),
                'R' => panic!("ENABLED: Reserved"),
                'S' => println!("ENABLED: Supervisor mode implemented"),
                'T' => panic!("ENABLED: Reserved"),
                'U' => println!("ENABLED: User mode implemented"),
                'V' => println!("Tentatively reserved for Vector extension"),
                'W' => panic!("ENABLED: Reserved"),
                'X' => println!("Non-standard extensions present"),
                'Y' => panic!("ENABLED: Reserved"),
                'Z' => panic!("ENABLED: Reserved"),
                _ => (),
            }
        }
    }
    Some(val)
}

fn read_mvendorid() -> Option<u32> {
    let mut val: u32;
    unsafe {
        asm!("csrr {}, mvendorid", out(reg) val);
    }
    if val == 0 {
        return None;
    }
    Some(val)
}

fn read_marchid() -> Option<usize> {
    let mut val: usize;
    unsafe {
        asm!("csrr {}, marchid", out(reg) val);
    }
    if val == 0 {
        return None;
    }
    Some(val)
}

fn read_mimpid() -> Option<usize> {
    let mut val: usize;
    unsafe {
        asm!("csrr {}, mimpid", out(reg) val);
    }
    if val == 0 {
        return None;
    }
    Some(val)
}
fn read_mhartid() -> usize {
    let mut val: usize;
    unsafe {
        asm!("csrr {}, mhartid", out(reg) val);
    }
    val
}

pub fn display_csr_infos() {
    if let Some(misa) = read_misa() {
        println!("misa      = {:#034x}", misa);
    } else {
        println!("misa CSR is zero, unsupported, unable to determine ISA version");
    }
    if let Some(mvendorid) = read_mvendorid() {
        println!("mvendorid = {:#034x}", mvendorid);
    } else {
        println!("mvendorid all zeros, not supported");
    }
    if let Some(marchid) = read_marchid() {
        println!("marchid   = {:#034x}", marchid);
    } else {
        println!("marchid all zeros, not supported");
    }
    if let Some(mimpid) = read_mimpid() {
        println!("mimpid    = {:#034x}", mimpid);
    } else {
        println!("mipmid all zeros, not supported");
    }

    let mhartid = read_mhartid();
    println!("curr mhardid = {}", mhartid);
}
