fn unicode(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.unicode(yes);
        self
    }