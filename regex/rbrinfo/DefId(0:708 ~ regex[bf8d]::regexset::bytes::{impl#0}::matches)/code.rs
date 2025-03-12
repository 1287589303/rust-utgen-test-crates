pub fn matches(&self, haystack: &[u8]) -> SetMatches {
        self.matches_at(haystack, 0)
    }