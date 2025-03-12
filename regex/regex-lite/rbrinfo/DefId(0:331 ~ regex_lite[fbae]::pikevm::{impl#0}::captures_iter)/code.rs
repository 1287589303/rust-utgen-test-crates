pub(crate) fn captures_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> CapturesMatches<'r, 'h> {
        // OK because the NFA wouldn't have compiled if this could overflow.
        let len = self.nfa().group_len().checked_mul(2).unwrap();
        CapturesMatches {
            it: FindMatches {
                pikevm: self,
                cache,
                haystack,
                at: 0,
                slots: vec![None; len],
                last_match_end: None,
            },
        }
    }