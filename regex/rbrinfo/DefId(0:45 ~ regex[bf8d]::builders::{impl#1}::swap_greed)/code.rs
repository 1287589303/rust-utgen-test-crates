fn swap_greed(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.swap_greed(yes);
        self
    }