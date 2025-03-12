fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            BuildErrorKind::Syntax { ref err, .. } => Some(err),
            BuildErrorKind::NFA(ref err) => Some(err),
        }
    }