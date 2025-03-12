fn too_many_states(limit: u64) -> BuildError {
        BuildError { kind: BuildErrorKind::TooManyStates { limit } }
    }