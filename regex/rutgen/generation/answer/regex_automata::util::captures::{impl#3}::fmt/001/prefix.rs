// Answer 0

#[test]
fn test_captures_debug_map_valid() {
    let group_info = GroupInfo::default(); // Use appropriate initialization based on context
    let slots: Vec<Option<NonMaxUsize>> = vec![Some(NonMaxUsize::new(1).unwrap()), Some(NonMaxUsize::new(2).unwrap())];
    let captures = Captures {
        group_info: group_info.clone(),
        pid: Some(PatternID(SmallIndex::new(0))),
        slots,
    };
    let debug_map = CapturesDebugMap { pid: PatternID(SmallIndex::new(0)), caps: &captures };
    let _ = format!("{:?}", debug_map);
}

#[test]
fn test_captures_debug_map_invalid() {
    let group_info = GroupInfo::default(); // Use appropriate initialization based on context
    let slots: Vec<Option<NonMaxUsize>> = vec![None, None];
    let captures = Captures {
        group_info: group_info.clone(),
        pid: Some(PatternID(SmallIndex::new(0))),
        slots,
    };
    let debug_map = CapturesDebugMap { pid: PatternID(SmallIndex::new(0)), caps: &captures };
    let _ = format!("{:?}", debug_map);
}

