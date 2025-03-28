pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<BoundedBacktracker, BuildError> {
        let nfa = self.thompson.build_many(patterns)?;
        self.build_from_nfa(nfa)
    }