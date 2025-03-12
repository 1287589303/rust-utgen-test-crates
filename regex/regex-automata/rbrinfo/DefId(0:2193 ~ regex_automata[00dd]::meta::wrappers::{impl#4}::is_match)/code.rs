pub(crate) fn is_match(
        &self,
        cache: &mut BoundedBacktrackerCache,
        input: &Input<'_>,
    ) -> bool {
        #[cfg(feature = "nfa-backtrack")]
        {
            // OK because we only permit access to this engine when we know
            // the haystack is short enough for the backtracker to run without
            // reporting an error.
            self.0
                .try_is_match(cache.0.as_mut().unwrap(), input.clone())
                .unwrap()
        }
        #[cfg(not(feature = "nfa-backtrack"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }