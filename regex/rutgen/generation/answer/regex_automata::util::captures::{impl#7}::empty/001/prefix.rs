// Answer 0

#[test]
fn test_group_info_empty() {
    let info = GroupInfo::empty();
    let pattern_len = info.pattern_len();
    let all_group_len = info.all_group_len();
    let slot_len = info.slot_len();
}

#[test]
fn test_group_info_new_empty_iterator() {
    let empty_iterator = core::iter::empty::<[Option<&str>; 0]>();
    let info = GroupInfo::new(empty_iterator).expect("should not fail on empty iterator");
    let pattern_len = info.pattern_len();
    let all_group_len = info.all_group_len();
    let slot_len = info.slot_len();
}

