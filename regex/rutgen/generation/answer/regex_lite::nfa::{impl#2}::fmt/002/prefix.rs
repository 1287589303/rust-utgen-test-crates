// Answer 0

#[test]
fn test_nfa_fmt_empty_pattern() {
    let nfa = NFA {
        pattern: String::new(),
        states: vec![State::Fail],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    let mut output = String::new();
    let _ = nfa.fmt(&mut core::fmt::Formatter::new());
}

#[test]
fn test_nfa_fmt_large_pattern() {
    let pattern = "a".repeat(256);
    let nfa = NFA {
        pattern,
        states: vec![State::Char { target: 1, ch: 'a' }],
        start: 0,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(5),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("group1")), Some(Arc::from("group2"))],
        memory_extra: 1024,
    };

    let mut output = String::new();
    let _ = nfa.fmt(&mut core::fmt::Formatter::new());
}

#[test]
fn test_nfa_fmt_pattern_with_capture_groups() {
    let pattern = "abc";
    let nfa = NFA {
        pattern: pattern.to_string(),
        states: vec![
            State::Char { target: 1, ch: 'a' },
            State::Char { target: 2, ch: 'b' },
            State::Char { target: 3, ch: 'c' },
            State::Match,
        ],
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(3),
        cap_name_to_index: {
            let mut map = CaptureNameMap::new();
            map.insert(Arc::from("group1"), 0);
            map.insert(Arc::from("group2"), 1);
            map
        },
        cap_index_to_name: vec![Some(Arc::from("group1")), Some(Arc::from("group2"))],
        memory_extra: 512,
    };

    let mut output = String::new();
    let _ = nfa.fmt(&mut core::fmt::Formatter::new());
}

#[test]
fn test_nfa_fmt_pattern_with_empty_states() {
    let pattern = "xyz";
    let nfa = NFA {
        pattern: pattern.to_string(),
        states: vec![State::Match],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 256,
    };

    let mut output = String::new();
    let _ = nfa.fmt(&mut core::fmt::Formatter::new());
}

