// Answer 0

#[test]
fn test_epsilon_closure_explore_with_splits_reverse() {
    let nfa = NFA {
        pattern: "ab".to_string(),
        states: vec![
            State::Splits { targets: vec![StateID(1)], reverse: true },
            State::Char { target: StateID(2), ch: 'b' },
            State::Match,
        ],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };

    let mut next = ActiveStates { 
        set: SparseSet::new(1), 
        slot_table: SlotTable::new(),
    };
    
    let mut stack = Vec::new();
    let mut curr_slots = vec![None; nfa.group_len()];
    let haystack: &[u8] = b"abc";
    let at = 0;
    let sid = StateID(0);

    let pike_vm = PikeVM::new(nfa);
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_explore_with_char() {
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![
            State::Char { target: StateID(1), ch: 'a' },
            State::Match,
        ],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };

    let mut next = ActiveStates { 
        set: SparseSet::new(1), 
        slot_table: SlotTable::new(),
    };
    
    let mut stack = Vec::new();
    let mut curr_slots = vec![None; nfa.group_len()];
    let haystack: &[u8] = b"abcd";
    let at = 0;
    let sid = StateID(0);

    let pike_vm = PikeVM::new(nfa);
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_explore_with_splits_non_reverse() {
    let nfa = NFA {
        pattern: "abc".to_string(),
        states: vec![
            State::Splits { targets: vec![StateID(1), StateID(2)], reverse: false },
            State::Char { target: StateID(2), ch: 'b' },
            State::Match,
        ],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };

    let mut next = ActiveStates { 
        set: SparseSet::new(1), 
        slot_table: SlotTable::new(),
    };
    
    let mut stack = Vec::new();
    let mut curr_slots = vec![None; nfa.group_len()];
    let haystack: &[u8] = b"abcd";
    let at = 0;
    let sid = StateID(0);

    let pike_vm = PikeVM::new(nfa);
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

