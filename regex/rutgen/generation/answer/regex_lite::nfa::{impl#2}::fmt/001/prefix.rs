// Answer 0

#[test]
fn test_fmt_with_empty_states() {
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: Vec::new(),
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    let _ = core::fmt::write(&mut core::fmt::Formatter::default(), format_args!("{}", nfa));
}

#[test]
fn test_fmt_with_single_state_char() {
    let nfa = NFA {
        pattern: "a".to_string(),
        states: vec![State::Char { target: 1, ch: 'a' }],
        start: 0,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 10,
    };

    let _ = core::fmt::write(&mut core::fmt::Formatter::default(), format_args!("{}", nfa));
}

#[test]
fn test_fmt_with_multiple_states() {
    let nfa = NFA {
        pattern: "xyz".to_string(),
        states: vec![
            State::Char { target: 1, ch: 'x' },
            State::Ranges { target: 2, ranges: vec![('y', 'y'), ('z', 'z')] },
            State::Match,
        ],
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::new("group".to_string()))],
        memory_extra: 20,
    };

    let _ = core::fmt::write(&mut core::fmt::Formatter::default(), format_args!("{}", nfa));
}

#[test]
fn test_fmt_with_state_fail() {
    let nfa = NFA {
        pattern: "fail".to_string(),
        states: vec![State::Fail],
        start: 1,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 5,
    };

    let _ = core::fmt::write(&mut core::fmt::Formatter::default(), format_args!("{}", nfa));
}

#[test]
fn test_fmt_with_large_memory_extra() {
    let nfa = NFA {
        pattern: "large".to_string(),
        states: vec![State::Match],
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![Some(Arc::from("first")), Some(Arc::from("second"))],
        memory_extra: usize::MAX,
    };

    let _ = core::fmt::write(&mut core::fmt::Formatter::default(), format_args!("{}", nfa));
}

