fn next(&mut self) -> Option<SmallIndex> {
        if self.rng.start >= self.rng.end {
            return None;
        }
        let next_id = self.rng.start + 1;
        let id = core::mem::replace(&mut self.rng.start, next_id);
        // new_unchecked is OK since we asserted that the number of
        // elements in this iterator will fit in an ID at construction.
        Some(SmallIndex::new_unchecked(id))
    }