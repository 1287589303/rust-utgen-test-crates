fn pattern_id(&self, state_index: usize, match_index: usize) -> PatternID {
        self.pattern_id_slice(state_index)[match_index]
    }