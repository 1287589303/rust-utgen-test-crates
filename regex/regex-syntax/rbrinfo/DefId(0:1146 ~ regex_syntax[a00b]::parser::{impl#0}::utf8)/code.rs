pub fn utf8(&mut self, yes: bool) -> &mut ParserBuilder {
        self.hir.utf8(yes);
        self
    }