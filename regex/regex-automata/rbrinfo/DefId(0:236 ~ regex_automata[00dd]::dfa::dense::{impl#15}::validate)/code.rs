fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {
        let sp = &dfa.special;
        for state in self.states() {
            // We check that the ID itself is well formed. That is, if it's
            // a special state then it must actually be a quit, dead, accel,
            // match or start state.
            if sp.is_special_state(state.id()) {
                let is_actually_special = sp.is_dead_state(state.id())
                    || sp.is_quit_state(state.id())
                    || sp.is_match_state(state.id())
                    || sp.is_start_state(state.id())
                    || sp.is_accel_state(state.id());
                if !is_actually_special {
                    // This is kind of a cryptic error message...
                    return Err(DeserializeError::generic(
                        "found dense state tagged as special but \
                         wasn't actually special",
                    ));
                }
                if sp.is_match_state(state.id())
                    && dfa.match_len(state.id()) == 0
                {
                    return Err(DeserializeError::generic(
                        "found match state with zero pattern IDs",
                    ));
                }
            }
            for (_, to) in state.transitions() {
                if !self.is_valid(to) {
                    return Err(DeserializeError::generic(
                        "found invalid state ID in transition table",
                    ));
                }
            }
        }
        Ok(())
    }