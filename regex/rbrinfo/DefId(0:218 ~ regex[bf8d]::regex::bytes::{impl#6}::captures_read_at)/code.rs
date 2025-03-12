pub fn captures_read_at<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h [u8],
        start: usize,
    ) -> Option<Match<'h>> {
        let input = Input::new(haystack).span(start..haystack.len());
        self.meta.search_captures(&input, &mut locs.0);
        locs.0.get_match().map(|m| Match::new(haystack, m.start(), m.end()))
    }