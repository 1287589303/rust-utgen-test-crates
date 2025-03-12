pub fn iter(&self) -> PatternSetIter<'_> {
        PatternSetIter { it: self.which.iter().enumerate() }
    }