pub fn matches_byte(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }