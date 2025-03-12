// Answer 0

#[test]
fn test_new_pikevm_empty_pattern() {
    let nfa = NFA {
        pattern: String::from(""),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
}

#[test]
fn test_new_pikevm_non_empty_pattern() {
    let nfa = NFA {
        pattern: String::from("abc"),
        states: vec![State::new()],
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1"))],
        memory_extra: 10,
    };
    let pikevm = PikeVM::new(nfa);
}

#[test]
fn test_new_pikevm_pattern_with_states() {
    let states = vec![State::new(), State::new()];
    let nfa = NFA {
        pattern: String::from("a?b"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("optional")), None],
        memory_extra: 5,
    };
    let pikevm = PikeVM::new(nfa);
}

#[test]
fn test_new_pikevm_complex_nfa() {
    let states = vec![State::new(), State::new(), State::new()];
    let nfa = NFA {
        pattern: String::from("(abc|def)"),
        states,
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group"))],
        memory_extra: 20,
    };
    let pikevm = PikeVM::new(nfa);
}

#[test]
fn test_new_pikevm_anchored_empty() {
    let nfa = NFA {
        pattern: String::from(""),
        states: vec![],
        start: 0,
        is_start_anchored: true,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pikevm = PikeVM::new(nfa);
}

