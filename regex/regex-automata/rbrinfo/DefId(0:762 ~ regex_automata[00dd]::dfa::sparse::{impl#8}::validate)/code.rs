fn validate(&self, sp: &Special) -> Result<Seen, DeserializeError> {
        let mut verified = Seen::new();
        // We need to make sure that we decode the correct number of states.
        // Otherwise, an empty set of transitions would validate even if the
        // recorded state length is non-empty.
        let mut len = 0;
        // We can't use the self.states() iterator because it assumes the state
        // encodings are valid. It could panic if they aren't.
        let mut id = DEAD;
        while id.as_usize() < self.sparse().len() {
            // Before we even decode the state, we check that the ID itself
            // is well formed. That is, if it's a special state then it must
            // actually be a quit, dead, accel, match or start state.
            if sp.is_special_state(id) {
                let is_actually_special = sp.is_dead_state(id)
                    || sp.is_quit_state(id)
                    || sp.is_match_state(id)
                    || sp.is_start_state(id)
                    || sp.is_accel_state(id);
                if !is_actually_special {
                    // This is kind of a cryptic error message...
                    return Err(DeserializeError::generic(
                        "found sparse state tagged as special but \
                         wasn't actually special",
                    ));
                }
            }
            let state = self.try_state(sp, id)?;
            verified.insert(id);
            // The next ID should be the offset immediately following `state`.
            id = StateID::new(wire::add(
                id.as_usize(),
                state.write_to_len(),
                "next state ID offset",
            )?)
            .map_err(|err| {
                DeserializeError::state_id_error(err, "next state ID offset")
            })?;
            len += 1;
        }
        // Now that we've checked that all top-level states are correct and
        // importantly, collected a set of valid state IDs, we have all the
        // information we need to check that all transitions are correct too.
        //
        // Note that we can't use `valid_ids` to iterate because it will
        // be empty in no-std no-alloc contexts. (And yes, that means our
        // verification isn't quite as good.) We can use `self.states()`
        // though at least, since we know that all states can at least be
        // decoded and traversed correctly.
        for state in self.states() {
            // Check that all transitions in this state are correct.
            for i in 0..state.ntrans {
                let to = state.next_at(i);
                // For no-alloc, we just check that the state can decode. It is
                // technically possible that the state ID could still point to
                // a non-existent state even if it decodes (fuzzing proved this
                // to be true), but it shouldn't result in any memory unsafety
                // or panics in non-debug mode.
                #[cfg(not(feature = "alloc"))]
                {
                    let _ = self.try_state(sp, to)?;
                }
                #[cfg(feature = "alloc")]
                {
                    if !verified.contains(&to) {
                        return Err(DeserializeError::generic(
                            "found transition that points to a \
                             non-existent state",
                        ));
                    }
                }
            }
        }
        if len != self.state_len {
            return Err(DeserializeError::generic(
                "mismatching sparse state length",
            ));
        }
        Ok(verified)
    }