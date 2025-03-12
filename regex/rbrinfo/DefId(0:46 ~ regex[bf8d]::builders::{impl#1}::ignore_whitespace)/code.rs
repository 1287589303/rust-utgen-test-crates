fn ignore_whitespace(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.ignore_whitespace(yes);
        self
    }