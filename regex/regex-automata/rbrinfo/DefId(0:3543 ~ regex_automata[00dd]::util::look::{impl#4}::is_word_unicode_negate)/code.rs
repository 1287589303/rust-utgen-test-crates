pub fn is_word_unicode_negate(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {
        // This is pretty subtle. Why do we need to do UTF-8 decoding here?
        // Well... at time of writing, the is_word_char_{fwd,rev} routines will
        // only return true if there is a valid UTF-8 encoding of a "word"
        // codepoint, and false in every other case (including invalid UTF-8).
        // This means that in regions of invalid UTF-8 (which might be a
        // subset of valid UTF-8!), it would result in \B matching. While this
        // would be questionable in the context of truly invalid UTF-8, it is
        // *certainly* wrong to report match boundaries that split the encoding
        // of a codepoint. So to work around this, we ensure that we can decode
        // a codepoint on either side of `at`. If either direction fails, then
        // we don't permit \B to match at all.
        //
        // Now, this isn't exactly optimal from a perf perspective. We could
        // try and detect this in is_word_char::{fwd,rev}, but it's not clear
        // if it's worth it. \B is, after all, rarely used. Even worse,
        // is_word_char::{fwd,rev} could do its own UTF-8 decoding, and so this
        // will wind up doing UTF-8 decoding twice. Owch. We could fix this
        // with more code complexity, but it just doesn't feel worth it for \B.
        //
        // And in particular, we do *not* have to do this with \b, because \b
        // *requires* that at least one side of `at` be a "word" codepoint,
        // which in turn implies one side of `at` must be valid UTF-8. This in
        // turn implies that \b can never split a valid UTF-8 encoding of a
        // codepoint. In the case where one side of `at` is truly invalid UTF-8
        // and the other side IS a word codepoint, then we want \b to match
        // since it represents a valid UTF-8 boundary. It also makes sense. For
        // example, you'd want \b\w+\b to match 'abc' in '\xFFabc\xFF'.
        //
        // Note also that this is not just '!is_word_unicode(..)' like it is
        // for the ASCII case. For example, neither \b nor \B is satisfied
        // within invalid UTF-8 sequences.
        let word_before = at > 0
            && match utf8::decode_last(&haystack[..at]) {
                None | Some(Err(_)) => return Ok(false),
                Some(Ok(_)) => is_word_char::rev(haystack, at)?,
            };
        let word_after = at < haystack.len()
            && match utf8::decode(&haystack[at..]) {
                None | Some(Err(_)) => return Ok(false),
                Some(Ok(_)) => is_word_char::fwd(haystack, at)?,
            };
        Ok(word_before == word_after)
    }