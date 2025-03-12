pub fn haystack_too_long(len: usize) -> MatchError {
        MatchError::new(MatchErrorKind::HaystackTooLong { len })
    }