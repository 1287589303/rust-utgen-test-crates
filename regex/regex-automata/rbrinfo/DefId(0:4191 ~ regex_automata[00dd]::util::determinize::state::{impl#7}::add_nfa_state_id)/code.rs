pub(crate) fn add_nfa_state_id(&mut self, sid: StateID) {
        ReprVec(&mut self.repr)
            .add_nfa_state_id(&mut self.prev_nfa_state_id, sid)
    }