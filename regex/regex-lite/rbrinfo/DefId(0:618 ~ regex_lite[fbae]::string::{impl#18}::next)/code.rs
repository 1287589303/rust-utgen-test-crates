fn next(&mut self) -> Option<Match<'h>> {
        self.it.next().map(|(s, e)| Match::new(self.haystack, s, e))
    }