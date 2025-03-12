pub(crate) fn is_match(&self, haystack: &[u8], at: usize) -> bool {
        use self::Look::*;

        match *self {
            Start => at == 0,
            End => at == haystack.len(),
            StartLF => at == 0 || haystack[at - 1] == b'\n',
            EndLF => at == haystack.len() || haystack[at] == b'\n',
            StartCRLF => {
                at == 0
                    || haystack[at - 1] == b'\n'
                    || (haystack[at - 1] == b'\r'
                        && (at >= haystack.len() || haystack[at] != b'\n'))
            }
            EndCRLF => {
                at == haystack.len()
                    || haystack[at] == b'\r'
                    || (haystack[at] == b'\n'
                        && (at == 0 || haystack[at - 1] != b'\r'))
            }
            Word => {
                let word_before =
                    at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after =
                    at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before != word_after
            }
            WordNegate => {
                let word_before =
                    at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after =
                    at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before == word_after
            }
            WordStart => {
                let word_before =
                    at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after =
                    at < haystack.len() && utf8::is_word_byte(haystack[at]);
                !word_before && word_after
            }
            WordEnd => {
                let word_before =
                    at > 0 && utf8::is_word_byte(haystack[at - 1]);
                let word_after =
                    at < haystack.len() && utf8::is_word_byte(haystack[at]);
                word_before && !word_after
            }
            WordStartHalf => {
                let word_before =
                    at > 0 && utf8::is_word_byte(haystack[at - 1]);
                !word_before
            }
            WordEndHalf => {
                let word_after =
                    at < haystack.len() && utf8::is_word_byte(haystack[at]);
                !word_after
            }
        }
    }