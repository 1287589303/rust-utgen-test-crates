pub(crate) fn try_search_half_fwd(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {
        #[cfg(feature = "hybrid")]
        {
            let fwd = self.0.forward();
            let mut fwdcache = cache.0.as_mut().unwrap().as_parts_mut().0;
            fwd.try_search_fwd(&mut fwdcache, input).map_err(|e| e.into())
        }
        #[cfg(not(feature = "hybrid"))]
        {
            // Impossible to reach because this engine is never constructed
            // if the requisite features aren't enabled.
            unreachable!()
        }
    }