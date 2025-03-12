pub fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool {
        let word_before = at > 0 && utf8::is_word_byte(haystack[at - 1]);
        let word_after =
            at < haystack.len() && utf8::is_word_byte(haystack[at]);
        !word_before && word_after
    }