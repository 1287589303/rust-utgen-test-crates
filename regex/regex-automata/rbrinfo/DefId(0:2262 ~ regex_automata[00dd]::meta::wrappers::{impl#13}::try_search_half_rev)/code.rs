pub(crate) fn try_search_half_rev(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {
        #[cfg(feature = "dfa-build")]
        {
            use crate::dfa::Automaton;
            self.0.reverse().try_search_rev(&input).map_err(|e| e.into())
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }