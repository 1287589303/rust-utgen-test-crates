pub fn line_terminator(&mut self, byte: u8) -> &mut TranslatorBuilder {
        self.line_terminator = byte;
        self
    }