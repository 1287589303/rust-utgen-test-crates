pub fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool {
        !self.is_word_ascii(haystack, at)
    }