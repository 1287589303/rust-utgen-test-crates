pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<DFA, BuildError> {
        let nfa =
            self.thompson.build_many(patterns).map_err(BuildError::nfa)?;
        self.build_from_nfa(nfa)
    }