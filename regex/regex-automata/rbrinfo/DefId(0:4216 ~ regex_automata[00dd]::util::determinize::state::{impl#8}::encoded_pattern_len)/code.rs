fn encoded_pattern_len(&self) -> usize {
        if !self.has_pattern_ids() {
            return 0;
        }
        // This unwrap is OK since the total number of patterns is always
        // guaranteed to fit into a usize.
        usize::try_from(wire::read_u32(&self.0[9..13])).unwrap()
    }