fn from(merr: MatchError) -> RetryFailError {
        use crate::util::search::MatchErrorKind::*;

        match *merr.kind() {
            Quit { offset, .. } => RetryFailError::from_offset(offset),
            GaveUp { offset } => RetryFailError::from_offset(offset),
            // These can never occur because we avoid them by construction
            // or with higher level control flow logic. For example, the
            // backtracker's wrapper will never hand out a backtracker engine
            // when the haystack would be too long.
            HaystackTooLong { .. } | UnsupportedAnchored { .. } => {
                unreachable!("found impossible error in meta engine: {}", merr)
            }
        }
    }