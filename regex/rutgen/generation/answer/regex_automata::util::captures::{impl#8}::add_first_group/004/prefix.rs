// Answer 0

#[test]
#[should_panic]
fn test_add_first_group_invalid_pid_zero() {
    let mut group_info = GroupInfoInner::default();
    group_info.add_first_group(PatternID(SmallIndex(0)));
    group_info.add_first_group(PatternID(SmallIndex(0))); // This should panic
}

#[test]
#[should_panic]
fn test_add_first_group_invalid_pid_increment() {
    let mut group_info = GroupInfoInner::default();
    group_info.add_first_group(PatternID(SmallIndex(0)));
    group_info.add_first_group(PatternID(SmallIndex(1)));
    group_info.add_first_group(PatternID(SmallIndex(2))); // This should panic
}

#[test]
fn test_add_first_group_valid() {
    let mut group_info = GroupInfoInner::default();
    group_info.add_first_group(PatternID(SmallIndex(0))); // Valid
    group_info.add_first_group(PatternID(SmallIndex(1))); // Valid
}

#[test]
#[should_panic]
fn test_add_first_group_invalid_length() {
    let mut group_info = GroupInfoInner::default();
    group_info.add_first_group(PatternID(SmallIndex(0))); // Valid
    group_info.slot_ranges.push((SmallIndex(0), SmallIndex(1))); // Manipulate to cause inconsistency
    group_info.add_first_group(PatternID(SmallIndex(1))); // This should panic
}

#[test]
fn test_add_first_group_at_max_index() {
    let mut group_info = GroupInfoInner::default();
    group_info.add_first_group(PatternID(SmallIndex(0))); // Valid
    group_info.add_first_group(PatternID(SmallIndex(1))); // Valid
    group_info.add_first_group(PatternID(SmallIndex(2))); // Valid, pushing to small slot limit
}

