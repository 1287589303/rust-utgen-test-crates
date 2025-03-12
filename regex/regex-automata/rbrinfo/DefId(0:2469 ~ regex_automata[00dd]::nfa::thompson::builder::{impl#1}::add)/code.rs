fn add(&mut self, state: State) -> Result<StateID, BuildError> {
        let id = StateID::new(self.states.len())
            .map_err(|_| BuildError::too_many_states(self.states.len()))?;
        self.memory_states += state.memory_usage();
        self.states.push(state);
        self.check_size_limit()?;
        Ok(id)
    }