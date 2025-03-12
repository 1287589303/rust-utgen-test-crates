fn prev_state_id(&self, id: StateID) -> Option<StateID> {
        if id == DEAD {
            None
        } else {
            // CORRECTNESS: Since 'id' is not the first state, subtracting 1
            // is always valid.
            Some(StateID::new_unchecked(id.as_usize().checked_sub(1).unwrap()))
        }
    }