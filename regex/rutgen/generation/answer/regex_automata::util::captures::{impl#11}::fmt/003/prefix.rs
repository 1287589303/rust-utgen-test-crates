// Answer 0

#[test]
fn test_group_info_error_missing_groups() {
    use crate::util::primitives::SmallIndex;

    let pattern_id_valid = PatternID(SmallIndex::from_usize(0).unwrap());
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::MissingGroups { pattern: pattern_id_valid },
    };
    
    let _ = core::fmt::write(&mut String::new().into(), &error);
}

#[test]
fn test_group_info_error_missing_groups_boundary() {
    use crate::util::primitives::SmallIndex;

    let pattern_id_boundary = PatternID(SmallIndex::from_usize(usize::MAX).unwrap());
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::MissingGroups { pattern: pattern_id_boundary },
    };
    
    let _ = core::fmt::write(&mut String::new().into(), &error);
}

#[test]
fn test_group_info_error_missing_groups_large() {
    use crate::util::primitives::SmallIndex;

    let pattern_id_large = PatternID(SmallIndex::from_usize(1000).unwrap());
    let error = GroupInfoError {
        kind: GroupInfoErrorKind::MissingGroups { pattern: pattern_id_large },
    };
    
    let _ = core::fmt::write(&mut String::new().into(), &error);
}

