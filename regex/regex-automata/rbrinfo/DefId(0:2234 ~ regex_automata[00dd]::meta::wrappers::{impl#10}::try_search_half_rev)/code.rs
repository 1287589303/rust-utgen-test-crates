pub(crate) fn try_search_half_rev(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {
        #[cfg(feature = "hybrid")]
        {
            let rev = self.0.reverse();
            let mut revcache = cache.0.as_mut().unwrap().as_parts_mut().1;
            rev.try_search_rev(&mut revcache, input).map_err(|e| e.into())
        }
        #[cfg(not(feature = "hybrid"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }