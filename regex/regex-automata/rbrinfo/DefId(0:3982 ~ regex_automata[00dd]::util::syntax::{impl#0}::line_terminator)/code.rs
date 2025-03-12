pub fn line_terminator(mut self, byte: u8) -> Config {
        self.line_terminator = byte;
        self
    }