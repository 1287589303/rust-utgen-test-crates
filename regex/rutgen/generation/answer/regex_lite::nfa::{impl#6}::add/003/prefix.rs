// Answer 0

#[test]
fn test_add_state_char() {
    let config = Config { nest_limit: 10, size_limit: Some(1024) };
    let pattern = String::from("a");
    let compiler = Compiler { config, nfa: RefCell::new(NFA { 
        pattern, 
        states: vec![], 
        start: 0, 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None, 
        cap_name_to_index: CaptureNameMap::new(), 
        cap_index_to_name: vec![], 
        memory_extra: 0 
    })};
    let state = State::Char { target: 1, ch: 'a' };
    let _ = compiler.add(state);
}

#[test]
fn test_add_state_ranges() {
    let config = Config { nest_limit: 10, size_limit: Some(2048) };
    let pattern = String::from("a-z");
    let compiler = Compiler { config, nfa: RefCell::new(NFA { 
        pattern, 
        states: vec![], 
        start: 0, 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None, 
        cap_name_to_index: CaptureNameMap::new(), 
        cap_index_to_name: vec![], 
        memory_extra: 0 
    })};
    let state = State::Ranges { target: 1, ranges: vec![('a', 'z')] };
    let _ = compiler.add(state);
}

#[test]
fn test_add_state_splits() {
    let config = Config { nest_limit: 10, size_limit: Some(5120) };
    let pattern = String::from("split");
    let compiler = Compiler { config, nfa: RefCell::new(NFA { 
        pattern, 
        states: vec![], 
        start: 0, 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None, 
        cap_name_to_index: CaptureNameMap::new(), 
        cap_index_to_name: vec![], 
        memory_extra: 0 
    })};
    let state = State::Splits { targets: vec![1, 2], reverse: false };
    let _ = compiler.add(state);
}

#[test]
fn test_add_state_capture() {
    let config = Config { nest_limit: 10, size_limit: Some(1024) };
    let pattern = String::from("capture");
    let compiler = Compiler { config, nfa: RefCell::new(NFA { 
        pattern, 
        states: vec![], 
        start: 0, 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None, 
        cap_name_to_index: CaptureNameMap::new(), 
        cap_index_to_name: vec![], 
        memory_extra: 0 
    })};
    let state = State::Capture { target: 1, slot: 0 };
    let _ = compiler.add(state);
}

#[test]
fn test_add_state_fail() {
    let config = Config { nest_limit: 10, size_limit: Some(2048) };
    let pattern = String::from("fail");
    let compiler = Compiler { config, nfa: RefCell::new(NFA { 
        pattern, 
        states: vec![], 
        start: 0, 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None, 
        cap_name_to_index: CaptureNameMap::new(), 
        cap_index_to_name: vec![], 
        memory_extra: 0 
    })};
    let state = State::Fail;
    let _ = compiler.add(state);
}

#[test]
fn test_add_state_match() {
    let config = Config { nest_limit: 10, size_limit: Some(1024) };
    let pattern = String::from("match");
    let compiler = Compiler { config, nfa: RefCell::new(NFA { 
        pattern, 
        states: vec![], 
        start: 0, 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None, 
        cap_name_to_index: CaptureNameMap::new(), 
        cap_index_to_name: vec![], 
        memory_extra: 0 
    })};
    let state = State::Match;
    let _ = compiler.add(state);
}

