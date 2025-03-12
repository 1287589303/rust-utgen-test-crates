pub(crate) fn none() -> BoundedBacktrackerCache {
        #[cfg(feature = "nfa-backtrack")]
        {
            BoundedBacktrackerCache(None)
        }
        #[cfg(not(feature = "nfa-backtrack"))]
        {
            BoundedBacktrackerCache(())
        }
    }