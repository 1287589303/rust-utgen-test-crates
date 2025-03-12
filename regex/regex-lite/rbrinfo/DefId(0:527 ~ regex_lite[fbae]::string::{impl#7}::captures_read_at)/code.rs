pub fn captures_read_at<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h str,
        start: usize,
    ) -> Option<Match<'h>> {
        let mut cache = self.pool.get();
        let matched = self.pikevm.search(
            &mut cache,
            haystack.as_bytes(),
            start,
            haystack.len(),
            false,
            &mut locs.0,
        );
        if !matched {
            return None;
        }
        let (start, end) = locs.get(0).unwrap();
        Some(Match::new(haystack, start, end))
    }