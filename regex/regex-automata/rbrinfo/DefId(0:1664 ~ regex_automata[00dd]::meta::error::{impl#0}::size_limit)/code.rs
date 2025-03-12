pub fn size_limit(&self) -> Option<usize> {
        match self.kind {
            BuildErrorKind::NFA(ref err) => err.size_limit(),
            _ => None,
        }
    }