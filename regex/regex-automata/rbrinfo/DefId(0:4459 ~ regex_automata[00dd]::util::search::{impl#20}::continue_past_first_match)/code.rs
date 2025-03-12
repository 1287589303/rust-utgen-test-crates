pub(crate) fn continue_past_first_match(&self) -> bool {
        *self == MatchKind::All
    }