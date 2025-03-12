// Answer 0

#[test]
fn test_get_group_valid_non_overlapping_case() {
    let group_info = GroupInfo::new(vec![vec![Some("group1")], vec![Some("group2")]]).unwrap();
    let captures = Captures {
        group_info: group_info.clone(),
        pid: Some(PatternID(SmallIndex::new(0))),
        slots: vec![Some(NonMaxUsize::new(0).unwrap()), Some(NonMaxUsize::new(5).unwrap())],
    };
    let index = 0;
    let span = captures.get_group(index);
}

#[test]
fn test_get_group_valid_large_index_case() {
    let group_info = GroupInfo::new(vec![vec![Some("group1")], vec![Some("group2")]]).unwrap();
    let captures = Captures {
        group_info: group_info.clone(),
        pid: Some(PatternID(SmallIndex::new(0))),
        slots: vec![Some(NonMaxUsize::new(0).unwrap()), Some(NonMaxUsize::new(5).unwrap())],
    };
    let index = 1;
    let span = captures.get_group(index);
}

#[test]
fn test_get_group_invalid_checked_add_case() {
    let group_info = GroupInfo::new(vec![vec![Some("group1")], vec![Some("group2")]]).unwrap();
    let captures = Captures {
        group_info: group_info.clone(),
        pid: Some(PatternID(SmallIndex::new(0))),
        slots: vec![Some(NonMaxUsize::new(0).unwrap()), Some(NonMaxUsize::new(5).unwrap())],
    };
    let index = 2; // This should lead to `Err` in `checked_add(1)`
    let span = captures.get_group(index);
}

