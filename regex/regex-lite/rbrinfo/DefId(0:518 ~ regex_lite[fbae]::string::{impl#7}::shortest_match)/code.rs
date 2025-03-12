pub fn shortest_match(&self, haystack: &str) -> Option<usize> {
        self.shortest_match_at(haystack, 0)
    }