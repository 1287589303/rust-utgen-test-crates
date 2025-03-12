// Answer 0

#[test]
fn test_patch_with_state_match() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
        size_limit: None,
    };
    let pattern = String::from(".*");
    let compiler = Compiler::new(config, pattern);
    
    let nfa = NFA {
        pattern: String::from(".*"),
        states: vec![State::Match],
        start: 0,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    compiler.nfa.replace(nfa);
    
    let from: StateID = 0; // Valid index for State::Match
    let to: StateID = 1;   // Valid StateID

    let _ = compiler.patch(from, to);
}

#[test]
fn test_patch_with_state_match_memory_extra() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
        size_limit: Some(100),
    };
    let pattern = String::from("abc");
    let compiler = Compiler::new(config, pattern);
    
    let nfa = NFA {
        pattern: String::from("abc"),
        states: vec![State::Match],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 50,
    };

    compiler.nfa.replace(nfa);
    
    let from: StateID = 0; // Valid index for State::Match
    let to: StateID = 2;   // Another valid StateID

    let _ = compiler.patch(from, to);
}

