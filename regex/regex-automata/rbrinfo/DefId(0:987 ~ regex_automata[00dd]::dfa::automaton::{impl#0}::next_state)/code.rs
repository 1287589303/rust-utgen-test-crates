fn next_state(&self, current: StateID, input: u8) -> StateID {
        (**self).next_state(current, input)
    }