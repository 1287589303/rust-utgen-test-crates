// Answer 0

#[test]
fn test_into_nfa_empty_start_pattern() {
    let inner = Inner {
        start_pattern: vec![],
        states: vec![],
        byte_class_set: ByteClassSet::empty(),
        ..Default::default()
    };
    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_empty_states() {
    let inner = Inner {
        start_pattern: vec![StateID(Default::default())],
        states: vec![],
        byte_class_set: ByteClassSet::empty(),
        ..Default::default()
    };
    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_no_match_state() {
    let inner = Inner {
        start_pattern: vec![],
        states: vec![State {
            transitions: vec![],
            ..Default::default()
        }],
        byte_class_set: ByteClassSet::empty(),
        ..Default::default()
    };
    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_single_empty_state() {
    let inner = Inner {
        start_pattern: vec![],
        states: vec![State {
            transitions: vec![],
            ..Default::default()
        }],
        byte_class_set: ByteClassSet::empty(),
        ..Default::default()
    };
    let nfa = inner.into_nfa();
}

