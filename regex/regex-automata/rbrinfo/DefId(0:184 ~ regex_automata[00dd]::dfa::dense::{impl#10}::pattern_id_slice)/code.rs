pub(crate) fn pattern_id_slice(&self, id: StateID) -> &[PatternID] {
        assert!(self.is_match_state(id));
        self.ms.pattern_id_slice(self.match_state_index(id))
    }