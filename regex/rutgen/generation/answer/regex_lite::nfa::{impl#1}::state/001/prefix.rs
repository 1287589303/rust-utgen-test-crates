// Answer 0

#[test]
fn test_state_valid_index_zero() {
    let states = vec![State::Char { target: 0, ch: 'a' }];
    let nfa = NFA {
        pattern: String::from("a"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    let _state = nfa.state(0);
}

#[test]
fn test_state_valid_index_non_zero() {
    let states = vec![
        State::Char { target: 1, ch: 'a' },
        State::Match,
    ];
    let nfa = NFA {
        pattern: String::from("a"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    let _state = nfa.state(1);
}

#[test]
#[should_panic]
fn test_state_invalid_index_negative() {
    let states = vec![State::Char { target: 0, ch: 'a' }];
    let nfa = NFA {
        pattern: String::from("a"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    let _state = nfa.state(!0);
}

#[test]
#[should_panic]
fn test_state_invalid_index_out_of_bounds() {
    let states = vec![State::Char { target: 0, ch: 'a' }];
    let nfa = NFA {
        pattern: String::from("a"),
        states,
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    let _state = nfa.state(1);
}

