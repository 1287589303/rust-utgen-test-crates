pub fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder {
            self.builder.line_terminator(byte);
            self
        }