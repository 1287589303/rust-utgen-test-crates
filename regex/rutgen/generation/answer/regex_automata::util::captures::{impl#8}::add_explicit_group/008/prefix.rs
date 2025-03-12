// Answer 0

#[test]
fn test_add_explicit_group_with_valid_name() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex::new(0).unwrap());
    let group = SmallIndex::new(0).unwrap();
    let name = "valid_group_name".to_string();

    group_info.slot_ranges.push((SmallIndex::ZERO, SmallIndex::new(2).unwrap()));
    
    let result = group_info.add_explicit_group(pid, group, Some(name));
}

#[test]
fn test_add_explicit_group_with_larger_group_index() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex::new(1).unwrap());
    let group = SmallIndex::new(1).unwrap();
    let name = "another_group".to_string();

    group_info.slot_ranges.push((SmallIndex::ZERO, SmallIndex::new(2).unwrap()));
    
    let result = group_info.add_explicit_group(pid, group, Some(name));
}

#[test]
fn test_add_explicit_group_with_boundary_group_index() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex::new(2).unwrap());
    let group = SmallIndex::new(SmallIndex::LIMIT - 2).unwrap();
    let name = "boundary_group".to_string();

    group_info.slot_ranges.push((SmallIndex::ZERO, SmallIndex::new(SmallIndex::LIMIT - 1).unwrap()));
    
    let result = group_info.add_explicit_group(pid, group, Some(name));
}

