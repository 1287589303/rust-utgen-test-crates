fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            BuildErrorKind::NFA(ref err) => Some(err),
            _ => None,
        }
    }