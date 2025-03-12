unsafe fn next_state_unchecked(
        &self,
        current: StateID,
        input: u8,
    ) -> StateID {
        (**self).next_state_unchecked(current, input)
    }