// Answer 0

#[test]
fn test_small_slot_len_empty_slot_ranges() {
    let group_info = GroupInfoInner {
        slot_ranges: Vec::new(),
        ..Default::default()
    };
    group_info.small_slot_len();
}

#[test]
fn test_small_slot_len_single_range() {
    let group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex(0), SmallIndex(5))],
        ..Default::default()
    };
    group_info.small_slot_len();
}

#[test]
fn test_small_slot_len_multiple_ranges() {
    let group_info = GroupInfoInner {
        slot_ranges: vec![
            (SmallIndex(0), SmallIndex(3)),
            (SmallIndex(3), SmallIndex(7)),
            (SmallIndex(7), SmallIndex(10)),
        ],
        ..Default::default()
    };
    group_info.small_slot_len();
}

#[test]
fn test_small_slot_len_boundary_condition() {
    let max_small_index = SmallIndex(u32::MAX);
    let group_info = GroupInfoInner {
        slot_ranges: vec![
            (SmallIndex(0), SmallIndex(u32::MAX - 1)),
            (SmallIndex(u32::MAX - 1), max_small_index),
        ],
        ..Default::default()
    };
    group_info.small_slot_len();
}

#[test]
fn test_small_slot_len_with_same_end_index() {
    let group_info = GroupInfoInner {
        slot_ranges: vec![
            (SmallIndex(0), SmallIndex(3)),
            (SmallIndex(3), SmallIndex(3)),
            (SmallIndex(3), SmallIndex(5)),
        ],
        ..Default::default()
    };
    group_info.small_slot_len();
}

