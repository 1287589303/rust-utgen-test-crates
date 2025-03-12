pub fn is_match_at(&self, haystack: &str, start: usize) -> bool {
        let input =
            Input::new(haystack).earliest(true).span(start..haystack.len());
        self.meta.search_half(&input).is_some()
    }