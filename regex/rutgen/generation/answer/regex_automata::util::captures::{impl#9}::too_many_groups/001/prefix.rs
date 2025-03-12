// Answer 0

#[test]
fn test_too_many_groups_minimum_one() {
    let pattern = PatternID(SmallIndex::from_u32(0).unwrap());
    let minimum = 1;
    let error = GroupInfoError::too_many_groups(pattern, minimum);
}

#[test]
fn test_too_many_groups_minimum_two() {
    let pattern = PatternID(SmallIndex::from_u32(1).unwrap());
    let minimum = 2;
    let error = GroupInfoError::too_many_groups(pattern, minimum);
}

#[test]
fn test_too_many_groups_max_pattern_id() {
    let pattern = PatternID(SmallIndex::from_u32(u32::MAX).unwrap());
    let minimum = 3;
    let error = GroupInfoError::too_many_groups(pattern, minimum);
}

#[test]
fn test_too_many_groups_large_minimum() {
    let pattern = PatternID(SmallIndex::from_u32(2).unwrap());
    let minimum = 10;
    let error = GroupInfoError::too_many_groups(pattern, minimum);
}

