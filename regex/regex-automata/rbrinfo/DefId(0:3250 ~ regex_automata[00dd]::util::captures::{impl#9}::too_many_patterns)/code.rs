fn too_many_patterns(err: PatternIDError) -> GroupInfoError {
        GroupInfoError { kind: GroupInfoErrorKind::TooManyPatterns { err } }
    }