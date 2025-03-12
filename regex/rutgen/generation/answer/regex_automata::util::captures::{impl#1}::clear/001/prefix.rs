// Answer 0

#[test]
fn test_clear_with_empty_slots() {
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: Vec::new(),
    };
    captures.clear();
}

#[test]
fn test_clear_with_none_slots() {
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: Some(PatternID(SmallIndex(1))),
        slots: vec![None, None, None],
    };
    captures.clear();
}

#[test]
fn test_clear_with_some_nonmaxusize_slots() {
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: Some(PatternID(SmallIndex(2))),
        slots: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())), Some(NonMaxUsize(NonZeroUsize::new(2).unwrap()))],
    };
    captures.clear();
}

