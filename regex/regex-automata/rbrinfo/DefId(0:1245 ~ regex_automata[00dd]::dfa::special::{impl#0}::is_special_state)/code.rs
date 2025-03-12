pub(crate) fn is_special_state(&self, id: StateID) -> bool {
        id <= self.max
    }