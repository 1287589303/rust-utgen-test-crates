pub fn is_byte(self, byte: u8) -> bool {
        self.as_u8().map_or(false, |b| b == byte)
    }