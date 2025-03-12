fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            #[cfg(feature = "syntax")]
            BuildErrorKind::Syntax(ref err) => Some(err),
            BuildErrorKind::Captures(ref err) => Some(err),
            _ => None,
        }
    }