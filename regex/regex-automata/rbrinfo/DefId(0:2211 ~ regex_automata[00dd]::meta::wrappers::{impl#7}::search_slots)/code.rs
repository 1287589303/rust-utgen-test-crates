pub(crate) fn search_slots(
        &self,
        cache: &mut OnePassCache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        #[cfg(feature = "dfa-onepass")]
        {
            // OK because we only permit getting a OnePassEngine when we know
            // the search is anchored and thus an error cannot occur.
            self.0
                .try_search_slots(cache.0.as_mut().unwrap(), input, slots)
                .unwrap()
        }
        #[cfg(not(feature = "dfa-onepass"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }