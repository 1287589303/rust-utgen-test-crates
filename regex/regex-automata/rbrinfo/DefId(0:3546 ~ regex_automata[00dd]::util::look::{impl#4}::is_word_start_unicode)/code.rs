pub fn is_word_start_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {
        let word_before = is_word_char::rev(haystack, at)?;
        let word_after = is_word_char::fwd(haystack, at)?;
        Ok(!word_before && word_after)
    }