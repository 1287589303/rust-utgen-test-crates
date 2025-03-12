pub fn build_from_nfa(
        &self,
        nfa: NFA,
    ) -> Result<BoundedBacktracker, BuildError> {
        nfa.look_set_any().available().map_err(BuildError::word)?;
        Ok(BoundedBacktracker { config: self.config.clone(), nfa })
    }