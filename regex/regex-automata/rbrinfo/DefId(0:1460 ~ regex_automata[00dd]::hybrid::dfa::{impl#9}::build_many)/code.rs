pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<DFA, BuildError> {
        let nfa = self
            .thompson
            .clone()
            // We can always forcefully disable captures because DFAs do not
            // support them.
            .configure(
                thompson::Config::new()
                    .which_captures(thompson::WhichCaptures::None),
            )
            .build_many(patterns)
            .map_err(BuildError::nfa)?;
        self.build_from_nfa(nfa)
    }