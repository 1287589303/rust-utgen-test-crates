fn count(self) -> usize {
        // If all we care about is a count of matches, then we only need to
        // find the end position of each match. This can give us a 2x perf
        // boost in some cases, because it avoids needing to do a reverse scan
        // to find the start of a match.
        let FindMatches { re, mut cache, it } = self;
        // This does the deref for PoolGuard once instead of every iter.
        let cache = &mut *cache;
        it.into_half_matches_iter(
            |input| Ok(re.search_half_with(cache, input)),
        )
        .count()
    }