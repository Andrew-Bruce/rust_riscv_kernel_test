use spin::Mutex;

lazy_static::lazy_static! {
    pub static ref WRITER: Mutex<UartWriter> = Mutex::new(UartWriter {
        uart_addr : 0x1000_0000,
    });
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub struct UartWriter {
    uart_addr: usize,
}

impl UartWriter {
    fn uart_write_string(&mut self, string: &str) {
        for c in string.as_bytes() {
            let uart_ptr: *mut u8 = self.uart_addr as *mut u8;
            unsafe {
                uart_ptr.write_volatile(*c);
            }
        }
    }
}

impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.uart_write_string(s);
        Ok(())
    }
}
