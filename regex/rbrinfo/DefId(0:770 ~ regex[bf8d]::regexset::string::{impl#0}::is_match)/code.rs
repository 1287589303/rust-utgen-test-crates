pub fn is_match(&self, haystack: &str) -> bool {
        self.is_match_at(haystack, 0)
    }