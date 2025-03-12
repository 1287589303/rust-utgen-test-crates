// Answer 0

#[test]
fn test_add_state_success() {
    let config = Config { 
        size_limit: Some(1024) // set size limit higher than typical state memory usage
    };
    let nfa = NFA {
        pattern: String::new(),
        states: Vec::new(),
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let compiler = Compiler {
        config,
        nfa: RefCell::new(nfa),
    };
    let state = State::Char { target: 1, ch: 'a' }; 
    let _ = compiler.add(state);
}

#[test]
fn test_add_state_exceeding_size_limit() {
    let config = Config { 
        size_limit: Some(10) // set size limit lower than typical state memory usage
    };
    let nfa = NFA {
        pattern: String::new(),
        states: Vec::new(),
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let compiler = Compiler {
        config,
        nfa: RefCell::new(nfa),
    };
    let state = State::Splits { targets: vec![1, 2], reverse: false }; 
    let _ = compiler.add(state);
}

