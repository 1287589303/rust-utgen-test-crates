pub fn shortest_match_at(
        &self,
        haystack: &[u8],
        start: usize,
    ) -> Option<usize> {
        let input =
            Input::new(haystack).earliest(true).span(start..haystack.len());
        self.meta.search_half(&input).map(|hm| hm.offset())
    }