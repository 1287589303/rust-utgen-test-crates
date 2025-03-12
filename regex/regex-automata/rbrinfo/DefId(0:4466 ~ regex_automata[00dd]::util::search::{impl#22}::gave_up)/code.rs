pub fn gave_up(offset: usize) -> MatchError {
        MatchError::new(MatchErrorKind::GaveUp { offset })
    }