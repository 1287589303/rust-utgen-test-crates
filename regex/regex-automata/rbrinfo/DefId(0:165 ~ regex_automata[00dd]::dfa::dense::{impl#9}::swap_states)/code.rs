pub(crate) fn swap_states(&mut self, id1: StateID, id2: StateID) {
        self.tt.swap(id1, id2);
    }