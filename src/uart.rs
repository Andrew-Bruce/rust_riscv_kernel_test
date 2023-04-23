pub struct UartWriter {
    uart_addr: usize,
}

impl UartWriter {
    pub fn new(uart_addr: usize) -> UartWriter {
        let uart_ptr: *mut u8 = uart_addr as *mut u8;

        unsafe {
            uart_ptr.add(3).write_volatile(0b11); //set word length to 8
            uart_ptr.add(2).write_volatile(0b1); //enable fifo
            uart_ptr.add(1).write_volatile(0b1); //enable recieving buffer interrupts
        }

        UartWriter { uart_addr }
    }

    fn uart_write_byte(&mut self, byte: u8) {
        let uart_ptr: *mut u8 = self.uart_addr as *mut u8;
        unsafe {
            uart_ptr.add(0).write_volatile(byte);
        }
    }

    fn uart_write_string(&mut self, string: &str) {
        for c in string.as_bytes() {
            self.uart_write_byte(*c);
        }
    }

    pub fn uart_read_byte(&mut self) -> Option<u8> {
        let uart_ptr: *mut u8 = self.uart_addr as *mut u8;
        let has_data: bool = unsafe { uart_ptr.add(5).read_volatile() & 1 == 1 };
        if has_data {
            let data: u8 = unsafe { uart_ptr.add(0).read_volatile() };
            Some(data)
        } else {
            None
        }
    }
}

impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.uart_write_string(s);
        Ok(())
    }
}
