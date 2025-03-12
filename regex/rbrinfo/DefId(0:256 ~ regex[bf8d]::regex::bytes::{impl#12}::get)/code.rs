pub fn get(&self, i: usize) -> Option<Match<'h>> {
        self.caps
            .get_group(i)
            .map(|sp| Match::new(self.haystack, sp.start, sp.end))
    }