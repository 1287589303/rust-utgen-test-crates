pub fn find_at<'h>(
        &self,
        haystack: &'h str,
        start: usize,
    ) -> Option<Match<'h>> {
        let input = Input::new(haystack).span(start..haystack.len());
        self.meta
            .search(&input)
            .map(|m| Match::new(haystack, m.start(), m.end()))
    }