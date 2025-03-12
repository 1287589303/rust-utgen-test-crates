pub fn pattern(&self) -> Option<PatternID> {
        match self.kind {
            BuildErrorKind::Syntax { pid, .. } => Some(pid),
            _ => None,
        }
    }