// Answer 0

#[test]
fn test_reset_with_empty_nfa() {
    let nfa = NFA {
        pattern: String::from(""),
        states: vec![],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut active_states = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };
    active_states.reset(&pike_vm);
}

#[test]
fn test_reset_with_nfa_with_one_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::new()],
        start: 0,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None; 1],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut active_states = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };
    active_states.reset(&pike_vm);
}

#[test]
fn test_reset_with_nfa_with_multiple_states() {
    let nfa = NFA {
        pattern: String::from("abc"),
        states: vec![State::new(), State::new(), State::new()],
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None; 2],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut active_states = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };
    active_states.reset(&pike_vm);
}

#[test]
fn test_reset_with_large_nfa() {
    let nfa = NFA {
        pattern: String::from("x"),
        states: (0..u32::MAX.as_usize()).map(|_| State::new()).collect(),
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut active_states = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };
    active_states.reset(&pike_vm);
}

