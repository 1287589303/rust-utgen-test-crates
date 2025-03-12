// Answer 0

#[test]
fn test_get_group_by_name_with_non_existing_group() {
    let group_info = GroupInfo::new(vec![Some("first"), Some("last")]).unwrap();
    let captures = Captures {
        group_info,
        pid: Some(PatternID(SmallIndex(0))),
        slots: vec![Some(NonMaxUsize::new(0).unwrap()), Some(NonMaxUsize::new(5).unwrap())],
    };
    let result = captures.get_group_by_name("middle");
}

#[test]
fn test_get_group_by_name_with_valid_group() {
    let group_info = GroupInfo::new(vec![Some("first"), Some("last")]).unwrap();
    let captures = Captures {
        group_info,
        pid: Some(PatternID(SmallIndex(0))),
        slots: vec![Some(NonMaxUsize::new(0).unwrap()), Some(NonMaxUsize::new(5).unwrap())],
    };
    let result_first = captures.get_group_by_name("first");
    let result_last = captures.get_group_by_name("last");
}

#[test]
fn test_get_group_by_name_empty_captures() {
    let group_info = GroupInfo::empty();
    let captures = Captures {
        group_info,
        pid: None,
        slots: Vec::new(),
    };
    let result = captures.get_group_by_name("first");
}

