fn pattern_len(&self, state_index: usize) -> usize {
        self.slices()[state_index * 2 + 1].as_usize()
    }