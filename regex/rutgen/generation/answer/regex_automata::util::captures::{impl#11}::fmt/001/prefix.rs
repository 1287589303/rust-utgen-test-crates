// Answer 0

#[test]
fn test_group_info_error_duplicate_case_1() {
    let pattern_id = PatternID(SmallIndex::new(0).unwrap());
    let name = String::from("duplicate_name");
    let kind = GroupInfoErrorKind::Duplicate { pattern: pattern_id, name };
    let error = GroupInfoError { kind };
    let _ = core::fmt::write(&mut String::new(), &error);
}

#[test]
fn test_group_info_error_duplicate_case_2() {
    let pattern_id = PatternID(SmallIndex::new(1).unwrap());
    let name = String::from("another_duplicate");
    let kind = GroupInfoErrorKind::Duplicate { pattern: pattern_id, name };
    let error = GroupInfoError { kind };
    let _ = core::fmt::write(&mut String::new(), &error);
}

#[test]
fn test_group_info_error_duplicate_case_3() {
    let pattern_id = PatternID(SmallIndex::new(255).unwrap());
    let name = String::from("test_duplicate_pattern");
    let kind = GroupInfoErrorKind::Duplicate { pattern: pattern_id, name };
    let error = GroupInfoError { kind };
    let _ = core::fmt::write(&mut String::new(), &error);
}

#[test]
fn test_group_info_error_duplicate_case_4() {
    let pattern_id = PatternID(SmallIndex::new(127).unwrap());
    let name = String::from("valid_name");
    let kind = GroupInfoErrorKind::Duplicate { pattern: pattern_id, name };
    let error = GroupInfoError { kind };
    let _ = core::fmt::write(&mut String::new(), &error);
}

