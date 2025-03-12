pub(crate) fn dfa_exceeded_size_limit(limit: usize) -> BuildError {
        BuildError { kind: BuildErrorKind::DFAExceededSizeLimit { limit } }
    }