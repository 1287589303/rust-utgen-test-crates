fn count(self) -> usize {
        let CapturesMatches { re, mut cache, it, .. } = self;
        // This does the deref for PoolGuard once instead of every iter.
        let cache = &mut *cache;
        it.into_half_matches_iter(
            |input| Ok(re.search_half_with(cache, input)),
        )
        .count()
    }