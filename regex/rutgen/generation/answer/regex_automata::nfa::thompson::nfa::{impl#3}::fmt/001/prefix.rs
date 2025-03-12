// Answer 0

#[test]
fn test_fmt_empty_states() {
    let nfa = Inner {
        states: Vec::new(),
        start_anchored: StateID(SmallIndex::default()),
        start_unanchored: StateID(SmallIndex::default()),
        start_pattern: Vec::new(),
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };
    let result = nfa.fmt(&mut String::new());
}

#[test]
#[should_panic]
fn test_fmt_invalid_formatter() {
    struct InvalidFormatter;
    impl fmt::Write for InvalidFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let nfa = Inner {
        states: Vec::new(),
        start_anchored: StateID(SmallIndex::default()),
        start_unanchored: StateID(SmallIndex::default()),
        start_pattern: Vec::new(),
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };
    let _result = nfa.fmt(&mut InvalidFormatter);
}

#[test]
fn test_fmt_start_anchored_equals_start_unanchored() {
    let state_id = StateID(SmallIndex::default());
    let nfa = Inner {
        states: Vec::new(),
        start_anchored: state_id,
        start_unanchored: state_id,
        start_pattern: Vec::new(),
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };
    let result = nfa.fmt(&mut String::new());
}

#[test]
fn test_fmt_empty_start_pattern() {
    let nfa = Inner {
        states: Vec::new(),
        start_anchored: StateID(SmallIndex::default()),
        start_unanchored: StateID(SmallIndex::default()),
        start_pattern: Vec::new(),
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };
    let result = nfa.fmt(&mut String::new());
}

#[test]
fn test_fmt_empty_byte_classes() {
    let nfa = Inner {
        states: Vec::new(),
        start_anchored: StateID(SmallIndex::default()),
        start_unanchored: StateID(SmallIndex::default()),
        start_pattern: Vec::new(),
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };
    let result = nfa.fmt(&mut String::new());
}

