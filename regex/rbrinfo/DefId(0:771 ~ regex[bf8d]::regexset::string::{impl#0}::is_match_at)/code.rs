pub fn is_match_at(&self, haystack: &str, start: usize) -> bool {
        self.meta.is_match(Input::new(haystack).span(start..haystack.len()))
    }