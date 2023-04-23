#![no_std]
#![feature(panic_info_message)]
mod uart;

const SYSCON_ADDR: usize = 0x0010_0000;

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("KERNEL PANIC!!!!");
    if let Some(location) = info.location() {
        println!("line {}, file {}", location.line(), location.file());
    } else {
        println!("panic line and file location unknown");
    }
    if let Some(message) = info.message() {
        println!("Panic message: {}", message);
    } else {
        println!("no panic message");
    }
    if let Some(payload_string) = info.payload().downcast_ref::<&str>() {
        println!("payload: {}", payload_string);
    } else {
        println!("payload is not a &str");
    }
    println!("no further information, dying now");
    abort();
}

#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

lazy_static::lazy_static! {
    pub static ref WRITER: spin::Mutex<uart::UartWriter> = spin::Mutex::new(uart::UartWriter::new(0x1000_0000));
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

fn poweroff() {
    println!("poweroff now");
    let syscon_ptr: *mut u32 = SYSCON_ADDR as *mut u32;
    unsafe {
        syscon_ptr.write_volatile(0x5555);
    }
}

fn reboot() {
    println!("reboot now");
    let syscon_ptr: *mut u32 = SYSCON_ADDR as *mut u32;
    unsafe {
        syscon_ptr.write_volatile(0x7777);
    }
}

#[no_mangle]
extern "C" fn kmain() {
    println!("poopoo peepee 你好 早上好");
    loop {
        //println!("mmmmm");
        let poo = WRITER.lock().uart_read_byte();
        if poo.is_some() {
            println!("read char {}", poo.unwrap());
        } else {
            //println!("read nothing");
        }
    }
}
