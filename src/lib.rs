#![no_std]
#![feature(panic_info_message)]

mod memory_alloc;
mod mmu;
mod uart;

extern "C" {
    static MEMORY_START: usize;
    static MEMORY_END: usize;

    //.text
    static TEXT_START: usize;
    static TEXT_END: usize;
    //.rodata
    static RODATA_START: usize;
    static RODATA_END: usize;
    //.data
    static DATA_START: usize;
    static DATA_END: usize;

    //.bss
    static BSS_START: usize;
    static BSS_END: usize;

    static STACK_TOP: usize;
    static STACK_BOT: usize;
    static HEAP_START: usize;
    static HEAP_END: usize;
    static HEAP_SIZE: usize;

    //syscon mmio
    static SYSCON_ADDR: usize;
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
    unsafe {
        let syscon_ptr: *mut u32 = SYSCON_ADDR as *mut u32;
        syscon_ptr.write_volatile(0x5555);
    }
}

fn reboot() {
    println!("reboot now");
    unsafe {
        let syscon_ptr: *mut u32 = SYSCON_ADDR as *mut u32;
        syscon_ptr.write_volatile(0x7777);
    }
}

fn print_memory_layout() {
    println!(
        "Memory | {:#010x} -> {:#010x}",
        unsafe { MEMORY_START },
        unsafe { MEMORY_END }
    );
    println!(
        "Text   | {:#010x} -> {:#010x}",
        unsafe { TEXT_START },
        unsafe { TEXT_END }
    );
    println!(
        "ROdata | {:#010x} -> {:#010x}",
        unsafe { RODATA_START },
        unsafe { RODATA_END }
    );
    println!(
        "Data   | {:#010x} -> {:#010x}",
        unsafe { DATA_START },
        unsafe { DATA_END }
    );
    println!(
        "BSS    | {:#010x} -> {:#010x}",
        unsafe { BSS_START },
        unsafe { BSS_END }
    );
    println!(
        "Stack  | {:#010x} -> {:#010x}",
        unsafe { STACK_BOT },
        unsafe { STACK_TOP }
    );
    println!(
        "Heap   | {:#010x} -> {:#010x}",
        unsafe { HEAP_START },
        unsafe { HEAP_END }
    );
    assert!(unsafe { HEAP_SIZE == HEAP_END - HEAP_START });
}

fn memory_map_important_stuff(root_table: &mut mmu::sv39::PageTable) {
    let ranges = unsafe {
        [
            (TEXT_START, TEXT_END),
            (RODATA_START, RODATA_END),
            (DATA_START, DATA_END),
            (BSS_START, BSS_END),
            (STACK_BOT, STACK_TOP),
            (HEAP_START, HEAP_END),
        ]
    };

    for pair in ranges {
        mmu::memory_map_region(
            pair.0,
            pair.1,
            root_table,
            mmu::sv39::PteBits::Read.val() | mmu::sv39::PteBits::Execute.val(),
        );
    }
}

fn test_memory_map(root_table: &mut mmu::sv39::PageTable) {
    let ranges = unsafe {
        [
            (TEXT_START, TEXT_END),
            (RODATA_START, RODATA_END),
            (DATA_START, DATA_END),
            (BSS_START, BSS_END),
            (STACK_BOT, STACK_TOP),
            (HEAP_START, HEAP_END),
        ]
    };

    for pair in ranges {
        for addr in ((pair.0)..(pair.1)).step_by(123) {//test some random addresses
            assert!(addr == (mmu::sv39::virt_to_phys(addr, root_table).unwrap() as usize));
        }
    }
}
//program entry point
//assembly should jump to here, if everything goes right then now rust takes over
#[no_mangle]
extern "C" fn kmain() {
    print_memory_layout();
    println!(
        "早晨, 你好, Hello, Здра́вствуйте, नमस्कार, السّلام عليكم, UTF-8 supports all languages!"
    );

    println!("initializing memory management");
    memory_alloc::init();
    memory_alloc::print_page_allocation();

    println!("creating root table");
    let root_table: &mut mmu::sv39::PageTable = unsafe {
        (memory_alloc::zero_allocate_pages(1).unwrap() as *mut mmu::sv39::PageTable)
            .as_mut()
            .unwrap()
    };
    println!("initializing memory mapping");
    memory_map_important_stuff(root_table);
    println!("testing map integrity");
    test_memory_map(root_table);
    println!("done");

    loop {
        let uart_byte: Option<u8> = WRITER.lock().uart_read_byte();
        if let Some(byte) = uart_byte {
            println!("read char {}", byte);
            if byte == b'p' {
                poweroff();
            }
            if byte == b'r' {
                reboot();
            }
            if byte == b'b' {
                break;
            }
        } else {
            //println!("read nothing");
        }
    }

    println!("unmapping virtual memory");
    mmu::sv39::unmap(root_table);
    memory_alloc::deallocate_pages((root_table as *mut mmu::sv39::PageTable) as *mut u8);
    memory_alloc::print_page_allocation();
    println!("heap allocations on shutdown, should be zero pages allocated");
    poweroff();
}
