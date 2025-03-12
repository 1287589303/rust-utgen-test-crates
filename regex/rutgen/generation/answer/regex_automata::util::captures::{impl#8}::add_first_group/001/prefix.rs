// Answer 0

#[test]
fn test_add_first_group_initial_call() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex(0));
    group_info.add_first_group(pid);
}

#[test]
#[should_panic]
fn test_add_first_group_duplicate_call() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex(0));
    group_info.add_first_group(pid);
    group_info.add_first_group(pid);
}

#[test]
fn test_add_first_group_second_call() {
    let mut group_info = GroupInfoInner::default();
    let pid1 = PatternID(SmallIndex(0));
    group_info.add_first_group(pid1);
    let pid2 = PatternID(SmallIndex(1));
    group_info.add_first_group(pid2);
}

#[test]
#[should_panic]
fn test_add_first_group_invalid_next_pid() {
    let mut group_info = GroupInfoInner::default();
    let pid1 = PatternID(SmallIndex(0));
    group_info.add_first_group(pid1);
    let pid3 = PatternID(SmallIndex(2)); // This should panic
    group_info.add_first_group(pid3);
}

