pub(crate) fn too_many_states(given: usize) -> BuildError {
        let limit = StateID::LIMIT;
        BuildError { kind: BuildErrorKind::TooManyStates { given, limit } }
    }