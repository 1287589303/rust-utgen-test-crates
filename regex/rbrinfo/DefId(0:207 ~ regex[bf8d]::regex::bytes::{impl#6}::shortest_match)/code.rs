pub fn shortest_match(&self, haystack: &[u8]) -> Option<usize> {
        self.shortest_match_at(haystack, 0)
    }