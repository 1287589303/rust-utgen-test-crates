pub(crate) fn too_many_patterns(given: usize) -> BuildError {
        let limit = PatternID::LIMIT;
        BuildError { kind: BuildErrorKind::TooManyPatterns { given, limit } }
    }