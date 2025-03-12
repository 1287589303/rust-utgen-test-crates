pub(crate) fn find_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> FindMatches<'r, 'h> {
        FindMatches {
            pikevm: self,
            cache,
            haystack,
            at: 0,
            slots: vec![None, None],
            last_match_end: None,
        }
    }