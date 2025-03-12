pub(crate) fn try_search(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<Match>, RetryFailError> {
        #[cfg(feature = "dfa-build")]
        {
            self.0.try_search(input).map_err(|e| e.into())
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }