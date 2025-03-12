// Answer 0

#[test]
fn test_add_explicit_group_success() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex::new(0).unwrap()); // valid PatternID
    let group = SmallIndex::new(0).unwrap(); // valid SmallIndex
    group_info.slot_ranges.push((SmallIndex::ZERO, SmallIndex::new(0).unwrap())); // initialize with a range

    // Ensure that the name_to_index map exists for this PatternID
    group_info.name_to_index.push(CaptureNameMap::new());
    group_info.index_to_name.push(vec![]);

    let name = "unique_group_name"; // unique non-empty name

    let result = group_info.add_explicit_group(pid, group, Some(name));
    // The result is expected to be Ok(())
}

#[test]
fn test_add_explicit_group_success_boundary() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex::new(0).unwrap()); // valid PatternID
    let group = SmallIndex::new(SmallIndex::LIMIT - 1).unwrap(); // boundary SmallIndex
    group_info.slot_ranges.push((SmallIndex::ZERO, SmallIndex::new(SmallIndex::LIMIT - 1).unwrap())); // initialize with a range

    // Ensure that the name_to_index map exists for this PatternID
    group_info.name_to_index.push(CaptureNameMap::new());
    group_info.index_to_name.push(vec![]);

    let name = "boundary_group_name"; // unique non-empty name

    let result = group_info.add_explicit_group(pid, group, Some(name));
    // The result is expected to be Ok(())
}

#[test]
fn test_add_explicit_group_success_duplicate_check() {
    let mut group_info = GroupInfoInner::default();
    let pid = PatternID(SmallIndex::new(0).unwrap()); // valid PatternID
    let group = SmallIndex::new(1).unwrap(); // valid SmallIndex
    group_info.slot_ranges.push((SmallIndex::ZERO, SmallIndex::new(0).unwrap())); // initialize with a range

    // Ensure that the name_to_index map exists for this PatternID
    group_info.name_to_index.push(CaptureNameMap::new());
    group_info.index_to_name.push(vec![]);

    let name = "duplicate_check_name"; // unique non-empty name
    group_info.name_to_index[pid].insert(Arc::from(name), group); // Set up initial state

    let result = group_info.add_explicit_group(pid, group, Some(name));
    // The result is expected to be Ok(())
}

