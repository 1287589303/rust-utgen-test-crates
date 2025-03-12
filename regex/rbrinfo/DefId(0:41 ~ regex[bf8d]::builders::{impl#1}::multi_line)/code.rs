fn multi_line(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.multi_line(yes);
        self
    }