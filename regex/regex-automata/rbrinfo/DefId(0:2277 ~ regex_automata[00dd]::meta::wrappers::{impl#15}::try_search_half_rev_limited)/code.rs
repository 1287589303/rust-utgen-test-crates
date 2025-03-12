pub(crate) fn try_search_half_rev_limited(
        &self,
        cache: &mut ReverseHybridCache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {
        #[cfg(feature = "hybrid")]
        {
            let dfa = &self.0;
            let mut cache = cache.0.as_mut().unwrap();
            crate::meta::limited::hybrid_try_search_half_rev(
                dfa, &mut cache, input, min_start,
            )
        }
        #[cfg(not(feature = "hybrid"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }