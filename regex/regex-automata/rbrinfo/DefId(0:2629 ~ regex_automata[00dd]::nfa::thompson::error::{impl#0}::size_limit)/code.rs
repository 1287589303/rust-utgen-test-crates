pub fn size_limit(&self) -> Option<usize> {
        match self.kind {
            BuildErrorKind::ExceededSizeLimit { limit } => Some(limit),
            _ => None,
        }
    }