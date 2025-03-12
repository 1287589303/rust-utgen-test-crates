// Answer 0

#[test]
fn test_nfa_with_valid_pattern_and_states() {
    let pattern = "a*b".to_string();
    let states = vec![State::new(), State::new()];
    let start = StateID::new(0);
    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let _nfa_ref = pike_vm.nfa();
}

#[test]
fn test_nfa_with_empty_pattern_and_states() {
    let pattern = "".to_string();
    let states = vec![State::new()];
    let start = StateID::new(0);
    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let _nfa_ref = pike_vm.nfa();
}

#[test]
fn test_nfa_with_single_state() {
    let pattern = "c".to_string();
    let states = vec![State::new()];
    let start = StateID::new(0);
    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(0),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let _nfa_ref = pike_vm.nfa();
}

#[test]
fn test_nfa_with_different_start_state() {
    let pattern = "abc".to_string();
    let states = vec![State::new(), State::new(), State::new()];
    let start = StateID::new(1);
    let nfa = NFA {
        pattern,
        states,
        start,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(2),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let _nfa_ref = pike_vm.nfa();
}

