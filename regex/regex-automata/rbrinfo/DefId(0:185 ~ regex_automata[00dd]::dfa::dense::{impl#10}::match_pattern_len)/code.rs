pub(crate) fn match_pattern_len(&self, id: StateID) -> usize {
        assert!(self.is_match_state(id));
        self.ms.pattern_len(self.match_state_index(id))
    }