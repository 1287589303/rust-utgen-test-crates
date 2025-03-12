pub fn find_at<'h>(
        &self,
        haystack: &'h str,
        start: usize,
    ) -> Option<Match<'h>> {
        let mut cache = self.pool.get();
        let mut slots = [None, None];
        let matched = self.pikevm.search(
            &mut cache,
            haystack.as_bytes(),
            start,
            haystack.len(),
            false,
            &mut slots,
        );
        if !matched {
            return None;
        }
        let (start, end) = (slots[0].unwrap().get(), slots[1].unwrap().get());
        Some(Match::new(haystack, start, end))
    }