fn line_terminator(&mut self, byte: u8) -> &mut Builder {
        self.metac = self.metac.clone().line_terminator(byte);
        self.syntaxc = self.syntaxc.line_terminator(byte);
        self
    }