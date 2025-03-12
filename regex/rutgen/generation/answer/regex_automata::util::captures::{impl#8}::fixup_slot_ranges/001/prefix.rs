// Answer 0

#[test]
fn test_fixup_slot_ranges_with_valid_input() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::new(0).unwrap(), SmallIndex::new(10).unwrap())],
        name_to_index: vec![],
        index_to_name: vec![],
        memory_extra: 0,
    };
    let pid = PatternID(SmallIndex::new(0).unwrap());
    group_info.add_first_group(pid);
    group_info.fixup_slot_ranges().unwrap();
}

#[test]
fn test_fixup_slot_ranges_with_boundary_end() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::new(0).unwrap(), SmallIndex::MAX)],
        name_to_index: vec![],
        index_to_name: vec![],
        memory_extra: 0,
    };
    let pid = PatternID(SmallIndex::new(0).unwrap());
    group_info.add_first_group(pid);
    
    let result = group_info.fixup_slot_ranges();
    assert!(result.is_err());
}

#[test]
fn test_fixup_slot_ranges_with_small_end() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::new(5).unwrap(), SmallIndex::new(6).unwrap())],
        name_to_index: vec![],
        index_to_name: vec![],
        memory_extra: 0,
    };
    let pid = PatternID(SmallIndex::new(0).unwrap());
    group_info.add_first_group(pid);
    group_info.fixup_slot_ranges().unwrap();
}

#[test]
fn test_fixup_slot_ranges_multiple_patterns() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![
            (SmallIndex::new(0).unwrap(), SmallIndex::new(2).unwrap()),
            (SmallIndex::new(3).unwrap(), SmallIndex::new(5).unwrap()),
        ],
        name_to_index: vec![],
        index_to_name: vec![],
        memory_extra: 0,
    };
    let pid0 = PatternID(SmallIndex::new(0).unwrap());
    let pid1 = PatternID(SmallIndex::new(1).unwrap());
    group_info.add_first_group(pid0);
    group_info.add_first_group(pid1);
    
    group_info.fixup_slot_ranges().unwrap();
}

#[test]
#[should_panic]
fn test_fixup_slot_ranges_exceeding_bounds() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::MAX, SmallIndex::MAX)],
        name_to_index: vec![],
        index_to_name: vec![],
        memory_extra: 0,
    };
    let pid = PatternID(SmallIndex::new(0).unwrap());
    group_info.add_first_group(pid);
    
    group_info.fixup_slot_ranges().unwrap();
}

