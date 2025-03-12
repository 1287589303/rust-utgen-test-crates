fn case_insensitive(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.case_insensitive(yes);
        self
    }