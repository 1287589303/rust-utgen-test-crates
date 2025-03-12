pub fn is_word_start_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {
        // See `is_word_unicode_negate` for why we need to do this. We don't
        // need to do it for `is_word_start_unicode` because that guarantees
        // that the position matched falls on a valid UTF-8 boundary given
        // that the right side must be in \w.
        let word_before = at > 0
            && match utf8::decode_last(&haystack[..at]) {
                None | Some(Err(_)) => return Ok(false),
                Some(Ok(_)) => is_word_char::rev(haystack, at)?,
            };
        Ok(!word_before)
    }