// Answer 0

#[test]
fn test_group_info_error_first_must_be_unnamed_valid_pattern_id() {
    use crate::util::primitives::SmallIndex;
    use crate::util::primitives::PatternID;

    let valid_pattern_id = PatternID(SmallIndex(1));
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::FirstMustBeUnnamed {
            pattern: valid_pattern_id.clone(),
        }
    };
    let _ = format!("{}", error);
}

#[test]
fn test_group_info_error_first_must_be_unnamed_max_pattern_id() {
    use crate::util::primitives::SmallIndex;
    use crate::util::primitives::PatternID;

    let max_pattern_id = PatternID(SmallIndex(255)); // Assume 255 is MAX_PATTERN_ID
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::FirstMustBeUnnamed {
            pattern: max_pattern_id.clone(),
        }
    };
    let _ = format!("{}", error);
}

#[test]
#[should_panic]
fn test_group_info_error_first_must_be_unnamed_zero_pattern_id() {
    use crate::util::primitives::SmallIndex;
    use crate::util::primitives::PatternID;

    let zero_pattern_id = PatternID(SmallIndex(0)); // This should not be valid
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::FirstMustBeUnnamed {
            pattern: zero_pattern_id.clone(),
        }
    };
    let _ = format!("{}", error);
}

