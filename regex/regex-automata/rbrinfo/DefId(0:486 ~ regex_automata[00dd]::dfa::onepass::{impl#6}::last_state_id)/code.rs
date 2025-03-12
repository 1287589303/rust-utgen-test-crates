fn last_state_id(&self) -> StateID {
        // CORRECTNESS: A DFA table is always non-empty since it always at
        // least contains a DEAD state. Since every state has the same stride,
        // we can just compute what the "next" state ID would have been and
        // then subtract 1 from it.
        StateID::new_unchecked(
            (self.table.len() >> self.stride2()).checked_sub(1).unwrap(),
        )
    }