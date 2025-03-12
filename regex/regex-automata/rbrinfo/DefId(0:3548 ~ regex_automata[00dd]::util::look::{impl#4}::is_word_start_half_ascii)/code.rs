pub fn is_word_start_half_ascii(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> bool {
        let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
        !word_before
    }