// Answer 0

#[test]
fn test_source_first_must_be_unnamed() {
    use crate::util::primitives::PatternID;
    let pattern_id = PatternID::from(1); // Example PatternID value
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::FirstMustBeUnnamed { pattern: pattern_id },
    };
    let _result = error.source();
}

#[test]
fn test_source_first_must_be_unnamed_different_pattern_id() {
    use crate::util::primitives::PatternID;
    let pattern_id = PatternID::from(2); // Another example PatternID value
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::FirstMustBeUnnamed { pattern: pattern_id },
    };
    let _result = error.source();
}

