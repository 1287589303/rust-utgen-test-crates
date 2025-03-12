pub fn is_word_end_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {
        // See `is_word_unicode_negate` for why we need to do this. We don't
        // need to do it for `is_word_end_unicode` because that guarantees
        // that the position matched falls on a valid UTF-8 boundary given
        // that the left side must be in \w.
        let word_after = at < haystack.len()
            && match utf8::decode(&haystack[at..]) {
                None | Some(Err(_)) => return Ok(false),
                Some(Ok(_)) => is_word_char::fwd(haystack, at)?,
            };
        Ok(!word_after)
    }