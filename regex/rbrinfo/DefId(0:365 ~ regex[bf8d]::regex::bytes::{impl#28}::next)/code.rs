fn next(&mut self) -> Option<Option<Match<'h>>> {
        self.it.next().map(|group| {
            group.map(|sp| Match::new(self.haystack, sp.start, sp.end))
        })
    }