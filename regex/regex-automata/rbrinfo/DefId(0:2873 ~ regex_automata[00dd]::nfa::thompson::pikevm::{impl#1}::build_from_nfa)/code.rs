pub fn build_from_nfa(&self, nfa: NFA) -> Result<PikeVM, BuildError> {
        nfa.look_set_any().available().map_err(BuildError::word)?;
        Ok(PikeVM { config: self.config.clone(), nfa })
    }