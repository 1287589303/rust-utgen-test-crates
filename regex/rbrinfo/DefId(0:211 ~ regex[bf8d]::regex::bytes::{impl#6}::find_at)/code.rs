pub fn find_at<'h>(
        &self,
        haystack: &'h [u8],
        start: usize,
    ) -> Option<Match<'h>> {
        let input = Input::new(haystack).span(start..haystack.len());
        self.meta.find(input).map(|m| Match::new(haystack, m.start(), m.end()))
    }