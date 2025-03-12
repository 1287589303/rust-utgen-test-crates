pub fn line_terminator(self, byte: u8) -> Config {
        Config { line_terminator: Some(byte), ..self }
    }