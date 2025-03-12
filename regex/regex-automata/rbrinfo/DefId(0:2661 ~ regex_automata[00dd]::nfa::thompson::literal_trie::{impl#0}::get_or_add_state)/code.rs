fn get_or_add_state(
        &mut self,
        from: StateID,
        byte: u8,
    ) -> Result<StateID, BuildError> {
        let active = self.states[from].active_chunk();
        match active.binary_search_by_key(&byte, |t| t.byte) {
            Ok(i) => Ok(active[i].next),
            Err(i) => {
                // Add a new state and get its ID.
                let next = StateID::new(self.states.len()).map_err(|_| {
                    BuildError::too_many_states(self.states.len())
                })?;
                self.states.push(State::default());
                // Offset our position to account for all transitions and not
                // just the ones in the active chunk.
                let i = self.states[from].active_chunk_start() + i;
                let t = Transition { byte, next };
                self.states[from].transitions.insert(i, t);
                Ok(next)
            }
        }
    }