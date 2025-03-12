// Answer 0

#[test]
fn test_add_empty_with_initial_state() {
    let config = Config { nest_limit: 10 };
    let pattern = String::from("a");
    let compiler = Compiler {
        config,
        nfa: RefCell::new(NFA {
            pattern,
            states: Vec::new(),
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::new(),
            cap_index_to_name: Vec::new(),
            memory_extra: 0,
        }),
    };
    let _ = compiler.add_empty();
}

#[test]
fn test_add_empty_exceeding_state_capacity() {
    let config = Config { nest_limit: 1 };
    let pattern = String::from("abc");
    let mut nfa_states = Vec::with_capacity(u32::MAX as usize - 1);
    for _ in 0..u32::MAX as usize - 1 {
        nfa_states.push(State::Fail);
    }
    let compiler = Compiler {
        config,
        nfa: RefCell::new(NFA {
            pattern,
            states: nfa_states,
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::new(),
            cap_index_to_name: Vec::new(),
            memory_extra: 0,
        }),
    };
    let result = compiler.add_empty();
    let _ = result; // To see if it results in an error due to exhausted state IDs
}

