use crate::println;
use core::arch::asm;

pub fn read_misa() {
    let mut val: usize;
    unsafe {
        asm!("csrr {}, misa", out(reg) val);
    }
    println!("misa register = {:#066b}", val);
    if val == 0 {
        println!("misa CSR is zero, unsupported, unable to determine ISA version");
        return;
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

    if xlen.is_none() {
        println!("invalid MXL value of {}, should be 1, 2, or 3", mxl);
        return;
    }

    let xlen: usize = xlen.unwrap();

    let zeros: usize = (val >> 26) & !(0b11 << ((xlen - 26) - 2));
    if zeros != 0 {
        println!("zeros of misa register isn't zero");
        println!("{:b}", zeros);
        println!("ignoring for now");
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
}
