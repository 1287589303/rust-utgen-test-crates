fn dot_matches_new_line(&mut self, yes: bool) -> &mut Builder {
        self.syntaxc = self.syntaxc.dot_matches_new_line(yes);
        self
    }