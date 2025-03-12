// Answer 0

#[test]
fn test_captures_clear_with_non_empty_slots() {
    let group_info = GroupInfo::default();
    let pattern_id = Some(PatternID(SmallIndex::default()));
    let slots = vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap()))];
    let mut captures = Captures {
        group_info,
        pid: pattern_id,
        slots,
    };

    captures.clear();
}

#[test]
fn test_captures_clear_with_multiple_slots() {
    let group_info = GroupInfo::default();
    let pattern_id = Some(PatternID(SmallIndex::default()));
    let slots = vec![
        Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())),
        Some(NonMaxUsize(NonZeroUsize::new(2).unwrap())),
        None,
    ];
    let mut captures = Captures {
        group_info,
        pid: pattern_id,
        slots,
    };

    captures.clear();
}

#[test]
fn test_captures_clear_with_boundary_slot_condition() {
    let group_info = GroupInfo::default();
    let pattern_id = Some(PatternID(SmallIndex::default()));
    let slots = vec![Some(NonMaxUsize(NonZeroUsize::new(usize::MAX).unwrap()))];
    let mut captures = Captures {
        group_info,
        pid: pattern_id,
        slots,
    };

    captures.clear();
}

#[test]
fn test_captures_clear_with_empty_slot_after_non_empty() {
    let group_info = GroupInfo::default();
    let pattern_id = Some(PatternID(SmallIndex::default()));
    let slots = vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())), None];
    let mut captures = Captures {
        group_info,
        pid: pattern_id,
        slots,
    };
    
    captures.clear();
}

