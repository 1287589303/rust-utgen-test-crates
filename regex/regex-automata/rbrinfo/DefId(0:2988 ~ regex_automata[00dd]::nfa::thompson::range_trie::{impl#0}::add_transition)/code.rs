fn add_transition(
        &mut self,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {
        self.state_mut(from_id)
            .transitions
            .push(Transition { range, next_id });
    }