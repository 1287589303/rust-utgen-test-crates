fn pattern_id_slice(&self, state_index: usize) -> &[PatternID] {
        let start = self.slices()[state_index * 2].as_usize();
        let len = self.pattern_len(state_index);
        &self.pattern_ids()[start..start + len]
    }