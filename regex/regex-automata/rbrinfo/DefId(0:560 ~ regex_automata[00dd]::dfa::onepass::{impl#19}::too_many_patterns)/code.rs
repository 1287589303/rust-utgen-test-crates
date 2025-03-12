fn too_many_patterns(limit: u64) -> BuildError {
        BuildError { kind: BuildErrorKind::TooManyPatterns { limit } }
    }