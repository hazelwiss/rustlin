pub unsafe trait ByteSerial {
    /// Put a byte into serial.
    unsafe fn putb(&self, byte: u8);
    /// Get a byte from serial.
    unsafe fn getb(&self) -> u8;

    /// Put stream of bytes into serial.
    unsafe fn putbs(&self, bytes: &[u8]) {
        for byte in bytes {
            self.putb(*byte)
        }
    }

    /// Put a character into the serial.
    unsafe fn putc(&self, c: char) {
        self.putb(c as u8);
    }

    /// Puts a string onto the serial.
    unsafe fn puts(&self, str: &str) {
        self.putbs(str.as_bytes());
    }

    /// Get a character from the serial.
    unsafe fn getc(&self) -> char {
        self.getb() as char
    }

    /// Get a string from the serial.
    unsafe fn gets(&self) -> &'static str {
        todo!()
    }
}
