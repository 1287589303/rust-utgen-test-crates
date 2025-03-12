pub fn is_match_at(&self, haystack: &[u8], start: usize) -> bool {
        self.meta.is_match(Input::new(haystack).span(start..haystack.len()))
    }