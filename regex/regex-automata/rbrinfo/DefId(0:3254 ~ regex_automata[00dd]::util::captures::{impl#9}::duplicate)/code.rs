fn duplicate(pattern: PatternID, name: &str) -> GroupInfoError {
        GroupInfoError {
            kind: GroupInfoErrorKind::Duplicate {
                pattern,
                name: String::from(name),
            },
        }
    }