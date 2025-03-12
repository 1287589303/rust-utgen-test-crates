// Answer 0

#[test]
fn test_patch_state_char_with_valid_transition() {
    let config = Config { nest_limit: 10, size_limit: Some(usize::MAX), flags: Flags::default() };
    let nfa = NFA {
        pattern: String::from(""),
        states: vec![State::Char { target: 0, ch: 'a' }, State::Fail],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    let compiler = Compiler { config, nfa: RefCell::new(nfa) };

    let result = compiler.patch(0, 1);
    result.unwrap();
}

#[test]
fn test_patch_state_char_increases_memory_extra() {
    let config = Config { nest_limit: 10, size_limit: Some(usize::MAX), flags: Flags::default() };
    let nfa = NFA {
        pattern: String::from(""),
        states: vec![State::Char { target: 1, ch: 'b' }, State::Splits { targets: vec![], reverse: false }],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    let compiler = Compiler { config, nfa: RefCell::new(nfa) };

    let result = compiler.patch(0, 2);
    result.unwrap();
}

