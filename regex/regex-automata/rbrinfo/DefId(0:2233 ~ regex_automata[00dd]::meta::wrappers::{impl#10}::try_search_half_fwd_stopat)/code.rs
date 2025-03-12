pub(crate) fn try_search_half_fwd_stopat(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Result<HalfMatch, usize>, RetryFailError> {
        #[cfg(feature = "hybrid")]
        {
            let dfa = self.0.forward();
            let mut cache = cache.0.as_mut().unwrap().as_parts_mut().0;
            crate::meta::stopat::hybrid_try_search_half_fwd(
                dfa, &mut cache, input,
            )
        }
        #[cfg(not(feature = "hybrid"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }