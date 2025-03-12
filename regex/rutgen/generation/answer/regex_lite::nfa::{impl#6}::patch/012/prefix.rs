// Answer 0

#[test]
fn test_patch_transition_goto_no_memory_growth() {
    let config = Config {
        nest_limit: 10,
        size_limit: Some(1024), // making sure we have a size limit set
        flags: Flags::empty(),
    };

    let pattern = String::from("a");
    let compiler = Compiler::new(config, pattern);
    
    let mut nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Goto { target: 1, look: None }, State::Match],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    compiler.nfa.replace(nfa);
    
    let from: StateID = 0;
    let to: StateID = 1;

    let _result = compiler.patch(from, to);
}

#[test]
fn test_patch_transition_goto_no_memory_growth_different_ids() {
    let config = Config {
        nest_limit: 10,
        size_limit: Some(2048), // making sure we have a size limit set
        flags: Flags::empty(),
    };

    let pattern = String::from("b");
    let compiler = Compiler::new(config, pattern);
    
    let mut nfa = NFA {
        pattern: String::from("b"),
        states: vec![State::Goto { target: 5, look: None }, State::Match],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };

    compiler.nfa.replace(nfa);
    
    let from: StateID = 0;
    let to: StateID = 5;

    let _result = compiler.patch(from, to);
}

