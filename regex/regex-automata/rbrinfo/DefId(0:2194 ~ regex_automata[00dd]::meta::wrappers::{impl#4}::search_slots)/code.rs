pub(crate) fn search_slots(
        &self,
        cache: &mut BoundedBacktrackerCache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        #[cfg(feature = "nfa-backtrack")]
        {
            // OK because we only permit access to this engine when we know
            // the haystack is short enough for the backtracker to run without
            // reporting an error.
            self.0
                .try_search_slots(cache.0.as_mut().unwrap(), input, slots)
                .unwrap()
        }
        #[cfg(not(feature = "nfa-backtrack"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }