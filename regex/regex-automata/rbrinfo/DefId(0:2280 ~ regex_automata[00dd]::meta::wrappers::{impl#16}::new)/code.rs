pub(crate) fn new(builder: &ReverseHybrid) -> ReverseHybridCache {
        #[cfg(feature = "hybrid")]
        {
            ReverseHybridCache(builder.0.as_ref().map(|e| e.0.create_cache()))
        }
        #[cfg(not(feature = "hybrid"))]
        {
            ReverseHybridCache(())
        }
    }