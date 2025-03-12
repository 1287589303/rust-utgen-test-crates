// Answer 0

#[test]
fn test_slot_with_valid_gid_zero() {
    let group_info = GroupInfo::default();
    let pid = PatternID::must(SmallIndex::new_unchecked(0).as_usize());
    let group_index = 0;
    let _ = group_info.slot(pid, group_index);
}

#[test]
fn test_slot_with_valid_gid_non_zero() {
    let group_info = GroupInfo {
        0: Arc::new(GroupInfoInner {
            slot_ranges: vec![(SmallIndex::new_unchecked(1), SmallIndex::new_unchecked(2))],
            ..Default::default()
        }),
    };
    let pid = PatternID::must(SmallIndex::new_unchecked(0).as_usize());
    let group_index = 1;
    let _ = group_info.slot(pid, group_index);
}

