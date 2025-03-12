pub(crate) fn too_many_match_pattern_ids() -> BuildError {
        BuildError { kind: BuildErrorKind::TooManyMatchPatternIDs }
    }