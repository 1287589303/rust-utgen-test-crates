// Answer 0

#[test]
fn test_slots_mut_non_empty_some() {
    use crate::util::primitives::NonMaxUsize;

    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: Some(PatternID(SmallIndex(0))),
        slots: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())),
                    Some(NonMaxUsize(NonZeroUsize::new(2).unwrap()))],
    };

    let slots_mut = captures.slots_mut();
}

#[test]
fn test_slots_mut_non_empty_none() {
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: Some(PatternID(SmallIndex(1))),
        slots: vec![None, Some(NonMaxUsize(NonZeroUsize::new(3).unwrap()))],
    };

    let slots_mut = captures.slots_mut();
}

#[test]
fn test_slots_mut_empty() {
    let mut captures = Captures {
        group_info: GroupInfo::default(),
        pid: None,
        slots: vec![],
    };

    let slots_mut = captures.slots_mut();
}

