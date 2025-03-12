pub fn syntax_error(&self) -> Option<&regex_syntax::Error> {
        match self.kind {
            BuildErrorKind::Syntax { ref err, .. } => Some(err),
            _ => None,
        }
    }