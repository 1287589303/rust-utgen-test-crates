pub fn matches_at(&self, haystack: &[u8], start: usize) -> SetMatches {
        let input = Input::new(haystack).span(start..haystack.len());
        let mut patset = PatternSet::new(self.meta.pattern_len());
        self.meta.which_overlapping_matches(&input, &mut patset);
        SetMatches(patset)
    }