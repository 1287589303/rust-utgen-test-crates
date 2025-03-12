fn missing_groups(pattern: PatternID) -> GroupInfoError {
        GroupInfoError { kind: GroupInfoErrorKind::MissingGroups { pattern } }
    }