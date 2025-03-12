pub(crate) fn try_search_half_fwd_stopat(
        &self,
        input: &Input<'_>,
    ) -> Result<Result<HalfMatch, usize>, RetryFailError> {
        #[cfg(feature = "dfa-build")]
        {
            let dfa = self.0.forward();
            crate::meta::stopat::dfa_try_search_half_fwd(dfa, input)
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }