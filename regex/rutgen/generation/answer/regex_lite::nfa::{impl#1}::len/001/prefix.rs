// Answer 0

#[test]
fn test_nfa_len_empty() {
    let nfa = NFA {
        pattern: String::from(""),
        states: Vec::new(),
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let _ = nfa.len();
}

#[test]
fn test_nfa_len_single_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Char { target: 1, ch: 'a' }],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let _ = nfa.len();
}

#[test]
fn test_nfa_len_multiple_states() {
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
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let _ = nfa.len();
}

#[test]
fn test_nfa_len_large_state_count() {
    let states: Vec<State> = (0..usize::MAX).map(|i| State::Char { target: i as StateID, ch: 'a' }).collect();
    let nfa = NFA {
        pattern: String::from("a"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let _ = nfa.len();
}

