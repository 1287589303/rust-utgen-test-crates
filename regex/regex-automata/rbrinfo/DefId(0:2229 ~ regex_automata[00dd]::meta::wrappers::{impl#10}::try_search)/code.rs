pub(crate) fn try_search(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Option<Match>, RetryFailError> {
        #[cfg(feature = "hybrid")]
        {
            let cache = cache.0.as_mut().unwrap();
            self.0.try_search(cache, input).map_err(|e| e.into())
        }
        #[cfg(not(feature = "hybrid"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }