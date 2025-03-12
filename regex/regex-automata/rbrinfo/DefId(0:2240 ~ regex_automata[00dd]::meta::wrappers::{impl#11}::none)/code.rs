pub(crate) fn none() -> HybridCache {
        #[cfg(feature = "hybrid")]
        {
            HybridCache(None)
        }
        #[cfg(not(feature = "hybrid"))]
        {
            HybridCache(())
        }
    }