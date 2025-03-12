pub fn read_matches_at(
        &self,
        matches: &mut [bool],
        haystack: &str,
        start: usize,
    ) -> bool {
        self.matches_read_at(matches, haystack, start)
    }