pub fn is_ascii(&self) -> bool {
        self.set.intervals().last().map_or(true, |r| r.end <= '\x7F')
    }