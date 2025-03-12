// Answer 0

#[test]
fn test_source_with_missing_groups() {
    use crate::util::primitives::PatternID;

    let pattern_id = PatternID::new(0); // Assuming a valid PatternID can be created this way
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::MissingGroups { pattern: pattern_id },
    };
    let _result = error.source();
}

#[test]
fn test_source_with_missing_groups_another_pattern_id() {
    use crate::util::primitives::PatternID;

    let pattern_id = PatternID::new(1); // Using another valid PatternID
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::MissingGroups { pattern: pattern_id },
    };
    let _result = error.source();
}

