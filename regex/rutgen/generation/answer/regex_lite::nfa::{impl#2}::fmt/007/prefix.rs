// Answer 0

#[test]
fn test_fmt_empty_states() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let mut output = Vec::new();
    let result = nfa.fmt(&mut output);
}

#[test]
fn test_fmt_single_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Char { target: 1, ch: 'a' }],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let mut output = Vec::new();
    let result = nfa.fmt(&mut output);
}

#[test]
fn test_fmt_multiple_states() {
    let nfa = NFA {
        pattern: String::from("abc"),
        states: vec![
            State::Char { target: 1, ch: 'a' },
            State::Char { target: 2, ch: 'b' },
            State::Char { target: 3, ch: 'c' },
        ],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let mut output = Vec::new();
    let result = nfa.fmt(&mut output);
}

#[test]
fn test_fmt_empty_pattern() {
    let nfa = NFA {
        pattern: String::from(""),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let mut output = Vec::new();
    let result = nfa.fmt(&mut output);
}

