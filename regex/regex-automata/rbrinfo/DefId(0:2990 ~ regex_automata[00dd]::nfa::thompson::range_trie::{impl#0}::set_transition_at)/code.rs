fn set_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {
        self.state_mut(from_id).transitions[i] = Transition { range, next_id };
    }