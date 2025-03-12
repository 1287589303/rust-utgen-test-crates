pub(crate) fn too_many_states() -> BuildError {
        BuildError { kind: BuildErrorKind::TooManyStates }
    }