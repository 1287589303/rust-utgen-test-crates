pub(crate) fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), RetryFailError> {
        #[cfg(feature = "dfa-build")]
        {
            use crate::dfa::Automaton;
            self.0
                .forward()
                .try_which_overlapping_matches(input, patset)
                .map_err(|e| e.into())
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }