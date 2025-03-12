// Answer 0

#[test]
fn test_start_with_min_state_id() {
    let nfa = NFA {
        pattern: String::from("abc"),
        states: vec![State::Char { target: 1, ch: 'a' }],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None; 0],
        memory_extra: 0,
    };
    let result = nfa.start();
}

#[test]
fn test_start_with_max_state_id() {
    let nfa = NFA {
        pattern: String::from("xyz"),
        states: vec![State::Char { target: 1, ch: 'x' }],
        start: 4294967295,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None; 0],
        memory_extra: 0,
    };
    let result = nfa.start();
}

#[test]
fn test_start_with_mixed_state_id() {
    let nfa = NFA {
        pattern: String::from("pattern"),
        states: vec![
            State::Char { target: 1, ch: 'p' },
            State::Char { target: 2, ch: 'a' },
            State::Char { target: 3, ch: 't' },
        ],
        start: 2,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::new(String::from("group1")))],
        memory_extra: 0,
    };
    let result = nfa.start();
}

