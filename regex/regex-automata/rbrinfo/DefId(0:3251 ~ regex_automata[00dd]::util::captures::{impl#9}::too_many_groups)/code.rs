fn too_many_groups(pattern: PatternID, minimum: usize) -> GroupInfoError {
        GroupInfoError {
            kind: GroupInfoErrorKind::TooManyGroups { pattern, minimum },
        }
    }