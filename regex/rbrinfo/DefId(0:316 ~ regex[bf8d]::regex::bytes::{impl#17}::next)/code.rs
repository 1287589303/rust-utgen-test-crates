fn next(&mut self) -> Option<Match<'h>> {
        self.it
            .next()
            .map(|sp| Match::new(self.haystack, sp.start(), sp.end()))
    }