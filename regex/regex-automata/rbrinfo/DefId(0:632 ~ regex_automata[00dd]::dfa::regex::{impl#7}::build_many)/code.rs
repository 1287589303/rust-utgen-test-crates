pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex, BuildError> {
        let forward = self.dfa.build_many(patterns)?;
        let reverse = self
            .dfa
            .clone()
            .configure(
                dense::Config::new()
                    .prefilter(None)
                    .specialize_start_states(false)
                    .start_kind(StartKind::Anchored)
                    .match_kind(MatchKind::All),
            )
            .thompson(crate::nfa::thompson::Config::new().reverse(true))
            .build_many(patterns)?;
        Ok(self.build_from_dfas(forward, reverse))
    }