pub(crate) fn try_search_half_rev_limited(
        &self,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {
        #[cfg(feature = "dfa-build")]
        {
            let dfa = &self.0;
            crate::meta::limited::dfa_try_search_half_rev(
                dfa, input, min_start,
            )
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }