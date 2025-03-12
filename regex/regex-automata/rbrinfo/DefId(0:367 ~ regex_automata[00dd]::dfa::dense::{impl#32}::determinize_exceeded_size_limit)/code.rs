pub(crate) fn determinize_exceeded_size_limit(limit: usize) -> BuildError {
        BuildError {
            kind: BuildErrorKind::DeterminizeExceededSizeLimit { limit },
        }
    }