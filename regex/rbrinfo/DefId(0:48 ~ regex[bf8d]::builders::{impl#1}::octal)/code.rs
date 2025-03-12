fn octal(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.octal(yes);
        self
    }