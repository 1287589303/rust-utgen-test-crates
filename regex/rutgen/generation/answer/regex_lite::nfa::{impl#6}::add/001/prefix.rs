// Answer 0

#[test]
fn test_add_exceeding_state_id() {
    let config = Config { 
        nest_limit: 10, 
        size_limit: Some(1024) 
    };
    let pattern = String::from("test_pattern");
    let compiler = Compiler {
        config,
        nfa: RefCell::new(NFA {
            pattern: pattern.clone(),
            states: vec![State::Char { target: 1, ch: 'a' }; (u32::MAX as usize)], // Create maximum states
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::new(),
            cap_index_to_name: vec![],
            memory_extra: 0,
        }),
    };
    let state = State::Char { target: 1, ch: 'b' };
    let _ = compiler.add(state);
}

#[test]
fn test_add_with_memory_usage_limit() {
    let config = Config { 
        nest_limit: 10, 
        size_limit: Some(10) 
    };
    let pattern = String::from("test_pattern");
    let compiler = Compiler {
        config,
        nfa: RefCell::new(NFA {
            pattern,
            states: vec![
                State::Char { target: 0, ch: 'a' },
                State::Splits { targets: vec![0], reverse: false },
            ],
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::new(),
            cap_index_to_name: vec![],
            memory_extra: 0,
        }),
    };
    let state = State::Char { target: 1, ch: 'b' };
    let _ = compiler.add(state);
}

#[test]
fn test_add_state_with_high_memory_usage() {
    let config = Config {
        nest_limit: 10,
        size_limit: Some(1024),
    };
    let pattern = String::from("test_pattern");
    let compiler = Compiler {
        config,
        nfa: RefCell::new(NFA {
            pattern,
            states: vec![
                State::Ranges { target: 0, ranges: vec![('a', 'z')] }, // Large ranges
                State::Char { target: 1, ch: 'b' },
            ],
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::new(),
            cap_index_to_name: vec![],
            memory_extra: 0,
        }),
    };
    let state = State::Capture { target: 1, slot: 2 };
    let _ = compiler.add(state);
}

