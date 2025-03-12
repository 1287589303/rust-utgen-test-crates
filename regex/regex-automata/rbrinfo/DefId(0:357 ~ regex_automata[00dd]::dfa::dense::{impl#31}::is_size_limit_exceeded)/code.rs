pub fn is_size_limit_exceeded(&self) -> bool {
        use self::BuildErrorKind::*;

        match self.kind {
            NFA(_) | Unsupported(_) => false,
            TooManyStates
            | TooManyStartStates
            | TooManyMatchPatternIDs
            | DFAExceededSizeLimit { .. }
            | DeterminizeExceededSizeLimit { .. } => true,
        }
    }