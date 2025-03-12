pub fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool {
        let word_after =
            at < haystack.len() && utf8::is_word_byte(haystack[at]);
        !word_after
    }