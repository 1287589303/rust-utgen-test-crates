pub(super) fn rev(
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, super::UnicodeWordBoundaryError> {
        Ok(match utf8::decode_last(&haystack[..at]) {
            None | Some(Err(_)) => false,
            Some(Ok(ch)) => try_is_word_character(ch).expect(
                "since unicode-word-boundary, syntax and unicode-perl \
                 are all enabled, it is expected that \
                 try_is_word_character succeeds",
            ),
        })
    }