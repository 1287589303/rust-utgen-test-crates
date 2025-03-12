fn reset(&mut self, _: &BoundedBacktracker) {
        self.bitset.truncate(0);
    }