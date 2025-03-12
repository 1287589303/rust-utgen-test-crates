fn crlf(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.crlf(yes);
        self
    }