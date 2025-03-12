// Answer 0

#[test]
fn test_fixup_slot_ranges_valid() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::new(0).unwrap(), SmallIndex::new(5).unwrap())],
        ..Default::default()
    };
    let pid = PatternID(SmallIndex::new(0).unwrap());
    group_info.add_first_group(pid);
    let _ = group_info.fixup_slot_ranges();
}

#[test]
fn test_fixup_slot_ranges_multiple() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![
            (SmallIndex::new(0).unwrap(), SmallIndex::new(4).unwrap()),
            (SmallIndex::new(6).unwrap(), SmallIndex::new(10).unwrap()),
        ],
        ..Default::default()
    };
    let pid = PatternID(SmallIndex::new(1).unwrap());
    group_info.add_first_group(pid);
    let _ = group_info.fixup_slot_ranges();
}

#[test]
fn test_fixup_slot_ranges_very_large() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::new(0).unwrap(), SmallIndex::new(SmallIndex::LIMIT as usize - 2).unwrap())],
        ..Default::default()
    };
    let pid = PatternID(SmallIndex::new(0).unwrap());
    group_info.add_first_group(pid);
    let _ = group_info.fixup_slot_ranges();
}

#[test]
fn test_fixup_slot_ranges_exceeding_limit() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::new(SmallIndex::LIMIT as usize - 4).unwrap(), SmallIndex::new(SmallIndex::LIMIT as usize - 1).unwrap())],
        ..Default::default()
    };
    let pid = PatternID(SmallIndex::new(0).unwrap());
    group_info.add_first_group(pid);
    let _ = group_info.fixup_slot_ranges();
}

#[test]
fn test_fixup_slot_ranges_boundary_condition() {
    let mut group_info = GroupInfoInner {
        slot_ranges: vec![(SmallIndex::new(1).unwrap(), SmallIndex::new(3).unwrap())],
        ..Default::default()
    };
    let pid = PatternID(SmallIndex::new(2).unwrap());
    group_info.add_first_group(pid);
    let _ = group_info.fixup_slot_ranges();
}

