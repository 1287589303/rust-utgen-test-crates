pub(crate) fn too_many_start_states() -> BuildError {
        BuildError { kind: BuildErrorKind::TooManyStartStates }
    }