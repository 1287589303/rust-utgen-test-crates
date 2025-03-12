pub fn is_match(&self, haystack: &[u8]) -> bool {
        self.is_match_at(haystack, 0)
    }