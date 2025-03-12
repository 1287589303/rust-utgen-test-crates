pub fn line_terminator(&mut self, byte: u8) -> &mut ParserBuilder {
        self.hir.line_terminator(byte);
        self
    }