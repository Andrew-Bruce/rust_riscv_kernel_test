#![no_std]
#![feature(panic_info_message)]
mod uart;

const SYSCON_ADDR: usize = 0x0010_0000;

extern "C" {
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}


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

//static variables can't be initialized by non-const functions, so we use lazy initalization to initalize them first time they are accessed
lazy_static::lazy_static! {
    //since uart is a raw pointer we should manually protect from multithreading with a mutex
    //for now use a simple spin lock but this should be changed to something more efficient later
    pub static ref WRITER: spin::Mutex<uart::UartWriter> = spin::Mutex::new(uart::UartWriter::new(0x1000_0000));
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

//make our own print!() and println!() will go to UART output, since the standard library and stdout don't exist
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

//program entry point
//assembly should jump to here, if everything goes right then now rust takes over
#[no_mangle]
extern "C" fn kmain() {
    /*
       println!(
       "早晨, 你好, Hello, Здра́вствуйте, नमस्कार, السّلام عليكم, UTF-8 supports all languages!"
       );
       */

    println!("hello");
    println!("asdf ");

    loop {
        let poo = WRITER.lock().uart_read_byte();
        if poo.is_some() {
            println!("read char {}", poo.unwrap());
            //println!("heap start address: {}", unsafe { HEAP_START} );    
            println!("mmmm");
        } else {
            println!("read nothing");
        }
    }
}
