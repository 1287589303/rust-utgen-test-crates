pub fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool {
        self.matches_inline(look, haystack, at)
    }