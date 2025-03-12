// Answer 0

#[test]
fn test_set_pattern_some() {
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![Some(NonMaxUsize(NonZeroUsize::new(0).unwrap())); 6],
    };
    
    captures.set_pattern(Some(PatternID(SmallIndex::new(0))));
    let is_match = captures.pid.is_some();
    let pattern_id = captures.pid;
    let slots: Vec<Option<NonMaxUsize>> = captures.slots().to_vec();
}

#[test]
fn test_set_pattern_none() {
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: Some(PatternID(SmallIndex::new(0))),
        slots: vec![Some(NonMaxUsize(NonZeroUsize::new(0).unwrap())); 6],
    };

    captures.set_pattern(None);
    let is_match = captures.pid.is_none();
    let pattern_id = captures.pid;
    let slots: Vec<Option<NonMaxUsize>> = captures.slots().to_vec();
}

