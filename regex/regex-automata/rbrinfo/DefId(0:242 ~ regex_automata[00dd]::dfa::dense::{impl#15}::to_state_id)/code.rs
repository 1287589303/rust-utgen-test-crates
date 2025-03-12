fn to_state_id(&self, index: usize) -> StateID {
        // CORRECTNESS: If the given index is not valid, then it is not
        // required for this to panic or return a valid state ID.
        StateID::new_unchecked(index << self.stride2)
    }