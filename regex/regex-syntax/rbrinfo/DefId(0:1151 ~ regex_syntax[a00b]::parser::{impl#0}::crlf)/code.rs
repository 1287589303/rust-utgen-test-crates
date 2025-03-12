pub fn crlf(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.crlf(yes);
        self
    }