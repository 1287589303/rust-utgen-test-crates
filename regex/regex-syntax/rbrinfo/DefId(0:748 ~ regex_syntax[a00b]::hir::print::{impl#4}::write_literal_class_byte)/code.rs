fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {
        if b <= 0x7F && !b.is_ascii_control() && !b.is_ascii_whitespace() {
            self.write_literal_char(char::try_from(b).unwrap())
        } else {
            write!(self.wtr, "\\x{:02X}", b)
        }
    }