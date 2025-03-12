pub(crate) fn none() -> ReverseHybridCache {
        #[cfg(feature = "hybrid")]
        {
            ReverseHybridCache(None)
        }
        #[cfg(not(feature = "hybrid"))]
        {
            ReverseHybridCache(())
        }
    }