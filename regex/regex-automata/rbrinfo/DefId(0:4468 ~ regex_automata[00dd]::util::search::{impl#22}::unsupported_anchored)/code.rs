pub fn unsupported_anchored(mode: Anchored) -> MatchError {
        MatchError::new(MatchErrorKind::UnsupportedAnchored { mode })
    }