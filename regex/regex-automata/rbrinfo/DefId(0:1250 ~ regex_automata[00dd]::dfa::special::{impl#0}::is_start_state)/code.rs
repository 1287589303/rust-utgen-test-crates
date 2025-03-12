pub(crate) fn is_start_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.min_start <= id && id <= self.max_start
    }