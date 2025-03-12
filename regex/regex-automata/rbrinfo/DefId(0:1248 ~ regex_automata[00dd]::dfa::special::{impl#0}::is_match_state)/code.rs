pub(crate) fn is_match_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.min_match <= id && id <= self.max_match
    }