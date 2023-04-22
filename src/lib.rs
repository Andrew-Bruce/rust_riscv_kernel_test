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



#[no_mangle]
extern "C"
fn kmain(){
    let uart_ptr = 0x1000_0000 as *mut u8;

    unsafe{
        uart_ptr.write_volatile(b'a');
    }

}
