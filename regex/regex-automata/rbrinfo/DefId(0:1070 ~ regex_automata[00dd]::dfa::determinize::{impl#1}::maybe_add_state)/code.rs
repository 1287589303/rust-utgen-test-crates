fn maybe_add_state(
        &mut self,
        builder: StateBuilderNFA,
    ) -> Result<(StateID, bool), BuildError> {
        if let Some(&cached_id) = self.cache.get(builder.as_bytes()) {
            // Since we have a cached state, put the constructed state's
            // memory back into our scratch space, so that it can be reused.
            self.put_state_builder(builder);
            return Ok((cached_id, false));
        }
        self.add_state(builder).map(|sid| (sid, true))
    }