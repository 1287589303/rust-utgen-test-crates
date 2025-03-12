fn add_state(
        &mut self,
        builder: StateBuilderNFA,
    ) -> Result<StateID, BuildError> {
        let id = self.dfa.add_empty_state()?;
        if !self.config.quit.is_empty() {
            for b in self.config.quit.iter() {
                self.dfa.set_transition(
                    id,
                    alphabet::Unit::u8(b),
                    self.dfa.quit_id(),
                );
            }
        }
        let state = builder.to_state();
        // States use reference counting internally, so we only need to count
        // their memory usage once.
        self.memory_usage_state += state.memory_usage();
        self.builder_states.push(state.clone());
        self.cache.insert(state, id);
        self.put_state_builder(builder);
        if let Some(limit) = self.config.dfa_size_limit {
            if self.dfa.memory_usage() > limit {
                return Err(BuildError::dfa_exceeded_size_limit(limit));
            }
        }
        if let Some(limit) = self.config.determinize_size_limit {
            if self.memory_usage() > limit {
                return Err(BuildError::determinize_exceeded_size_limit(
                    limit,
                ));
            }
        }
        Ok(id)
    }