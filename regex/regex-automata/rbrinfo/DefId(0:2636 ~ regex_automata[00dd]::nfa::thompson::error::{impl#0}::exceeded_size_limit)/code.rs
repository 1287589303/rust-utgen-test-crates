pub(crate) fn exceeded_size_limit(limit: usize) -> BuildError {
        BuildError { kind: BuildErrorKind::ExceededSizeLimit { limit } }
    }