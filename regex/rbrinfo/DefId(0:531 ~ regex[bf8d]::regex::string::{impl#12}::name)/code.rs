pub fn name(&self, name: &str) -> Option<Match<'h>> {
        self.caps
            .get_group_by_name(name)
            .map(|sp| Match::new(self.haystack, sp.start, sp.end))
    }