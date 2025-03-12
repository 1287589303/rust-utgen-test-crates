pub fn matches(&self, haystack: &str) -> SetMatches {
        self.matches_at(haystack, 0)
    }