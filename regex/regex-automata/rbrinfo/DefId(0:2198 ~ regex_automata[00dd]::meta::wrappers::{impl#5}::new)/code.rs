pub(crate) fn new(
        builder: &BoundedBacktracker,
    ) -> BoundedBacktrackerCache {
        #[cfg(feature = "nfa-backtrack")]
        {
            BoundedBacktrackerCache(
                builder.0.as_ref().map(|e| e.0.create_cache()),
            )
        }
        #[cfg(not(feature = "nfa-backtrack"))]
        {
            BoundedBacktrackerCache(())
        }
    }