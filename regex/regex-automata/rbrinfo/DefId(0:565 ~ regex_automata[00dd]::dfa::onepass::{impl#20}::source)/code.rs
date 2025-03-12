fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::BuildErrorKind::*;

        match self.kind {
            NFA(ref err) => Some(err),
            Word(ref err) => Some(err),
            _ => None,
        }
    }