// Answer 0

#[test]
fn test_duplicate_with_small_index_zero() {
    let pattern = PatternID(SmallIndex::from_usize(0).unwrap());
    let name = "duplicate_name_1";
    let _error = GroupInfoError::duplicate(pattern, name);
}

#[test]
fn test_duplicate_with_small_index_one() {
    let pattern = PatternID(SmallIndex::from_usize(1).unwrap());
    let name = "duplicate_name_2";
    let _error = GroupInfoError::duplicate(pattern, name);
}

#[test]
fn test_duplicate_with_small_index_two() {
    let pattern = PatternID(SmallIndex::from_usize(2).unwrap());
    let name = "duplicate_name_3";
    let _error = GroupInfoError::duplicate(pattern, name);
}

#[test]
fn test_duplicate_with_small_index_three() {
    let pattern = PatternID(SmallIndex::from_usize(3).unwrap());
    let name = "duplicate_name_4";
    let _error = GroupInfoError::duplicate(pattern, name);
}

#[test]
fn test_duplicate_with_small_index_four() {
    let pattern = PatternID(SmallIndex::from_usize(4).unwrap());
    let name = "duplicate_name_5";
    let _error = GroupInfoError::duplicate(pattern, name);
}

