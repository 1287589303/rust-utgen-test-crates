// Answer 0

#[test]
fn test_nfa_fmt_empty_states() {
    let mut formatter = std::fmt::Formatter::new();
    let nfa = Inner {
        states: Vec::new(),
        start_anchored: StateID(SmallIndex::new(0)),
        start_unanchored: StateID(SmallIndex::new(1)),
        start_pattern: vec![StateID(SmallIndex::new(0))],
        group_info: GroupInfo::default(),
        byte_class_set: ByteClassSet::default(),
        byte_classes: ByteClasses::default(),
        has_capture: false,
        has_empty: false,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        look_set_any: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        memory_extra: 0,
    };

    let _ = nfa.fmt(&mut formatter);
}

#[test]
fn test_nfa_fmt_single_pattern() {
    let mut formatter = std::fmt::Formatter::new();
    let nfa = Inner {
        states: Vec::new(),
        start_anchored: StateID(SmallIndex::new(0)),
        start_unanchored: StateID(SmallIndex::new(1)),
        start_pattern: vec![StateID(SmallIndex::new(0))],
        group_info: GroupInfo::default(),
        byte_class_set: ByteClassSet::default(),
        byte_classes: ByteClasses::default(),
        has_capture: false,
        has_empty: false,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        look_set_any: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        memory_extra: 0,
    };

    let _ = nfa.fmt(&mut formatter);
}

#[test]
fn test_nfa_fmt_invalid_byte_classes() {
    let mut formatter = std::fmt::Formatter::new();
    let nfa = Inner {
        states: Vec::new(),
        start_anchored: StateID(SmallIndex::new(0)),
        start_unanchored: StateID(SmallIndex::new(1)),
        start_pattern: vec![StateID(SmallIndex::new(0))],
        group_info: GroupInfo::default(),
        byte_class_set: ByteClassSet::default(),
        byte_classes: ByteClasses(Vec::new()),
        has_capture: false,
        has_empty: false,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        look_set_any: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        memory_extra: 0,
    };

    let result = nfa.fmt(&mut formatter);
    assert!(result.is_err());
}

