#![no_std]


#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    abort();
}

#[no_mangle]
extern "C"
fn abort() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}


fn uart_write_string(string: &str){
    let uart_ptr = 0x1000_0000 as *mut u8;

    for c in string.as_bytes(){
        unsafe{
            uart_ptr.write_volatile(*c);
        }
    }
}



#[no_mangle]
extern "C"
fn kmain(){
    uart_write_string("poopoo peepee 你好 早上好");
}
