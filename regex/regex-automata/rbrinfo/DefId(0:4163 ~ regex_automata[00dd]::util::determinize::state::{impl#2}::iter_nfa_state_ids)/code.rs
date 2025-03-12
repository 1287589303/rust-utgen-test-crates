pub(crate) fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F) {
        self.repr().iter_nfa_state_ids(f)
    }