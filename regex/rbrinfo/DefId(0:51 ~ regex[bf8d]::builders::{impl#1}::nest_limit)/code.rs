fn nest_limit(&mut self, limit: u32) -> &mut Builder {
        self.syntaxc = self.syntaxc.nest_limit(limit);
        self
    }