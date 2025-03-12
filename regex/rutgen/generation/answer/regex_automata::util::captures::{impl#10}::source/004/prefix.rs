// Answer 0

#[test]
fn test_source_too_many_groups() {
    use crate::util::primitives::{PatternID, GroupInfoError, GroupInfoErrorKind};

    let pattern_id = PatternID::new(1).unwrap(); // Assuming PatternID::new() initializes a valid ID
    let minimum_groups = 1;

    let error = GroupInfoError {
        kind: GroupInfoErrorKind::TooManyGroups {
            pattern: pattern_id,
            minimum: minimum_groups,
        },
    };

    let _result = error.source();
}

#[test]
fn test_source_too_many_groups_edge_case() {
    use crate::util::primitives::{PatternID, GroupInfoError, GroupInfoErrorKind};

    let pattern_id = PatternID::new(2).unwrap(); // Testing with another valid PatternID
    let minimum_groups = 100;

    let error = GroupInfoError {
        kind: GroupInfoErrorKind::TooManyGroups {
            pattern: pattern_id,
            minimum: minimum_groups,
        },
    };

    let _result = error.source();
}

