// Answer 0

#[test]
fn test_first_must_be_unnamed_minimum_pattern_id() {
    let pattern_id = PatternID(SmallIndex(0));
    let result = GroupInfoError::first_must_be_unnamed(pattern_id);
}

#[test]
fn test_first_must_be_unnamed_maximum_pattern_id() {
    let pattern_id = PatternID(SmallIndex(u32::MAX as usize));
    let result = GroupInfoError::first_must_be_unnamed(pattern_id);
}

