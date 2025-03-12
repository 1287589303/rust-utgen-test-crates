pub fn pattern(&self) -> Option<PatternID> {
        match *self {
            Anchored::Pattern(pid) => Some(pid),
            _ => None,
        }
    }