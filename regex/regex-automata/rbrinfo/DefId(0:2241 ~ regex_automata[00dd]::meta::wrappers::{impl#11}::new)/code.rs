pub(crate) fn new(builder: &Hybrid) -> HybridCache {
        #[cfg(feature = "hybrid")]
        {
            HybridCache(builder.0.as_ref().map(|e| e.0.create_cache()))
        }
        #[cfg(not(feature = "hybrid"))]
        {
            HybridCache(())
        }
    }