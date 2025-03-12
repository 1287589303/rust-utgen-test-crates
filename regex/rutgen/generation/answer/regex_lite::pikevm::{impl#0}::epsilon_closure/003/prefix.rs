// Answer 0

#[test]
fn test_epsilon_closure_with_valid_stack_and_slots() {
    let nfa = NFA {
        pattern: String::from("a*"),
        states: vec![State::Char { code: 'a' }, State::Match],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    
    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![FollowEpsilon::Explore(StateID(0))];
    let mut curr_slots = vec![None; 1];
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: SlotTable::new(),
    };
    let haystack = b"aa";
    let at = 0;
    let sid = StateID(0);
    
    pike_vm.epsilon_closure(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_with_multiple_slots() {
    let nfa = NFA {
        pattern: String::from("b*"),
        states: vec![State::Char { code: 'b' }, State::Match],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    
    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![FollowEpsilon::Explore(StateID(0))];
    let mut curr_slots = vec![None; 2];
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: SlotTable::new(),
    };
    let haystack = b"bbb";
    let at = 1;
    let sid = StateID(0);
    
    pike_vm.epsilon_closure(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_with_haystack_length_one() {
    let nfa = NFA {
        pattern: String::from("c"),
        states: vec![State::Char { code: 'c' }, State::Match],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };
    
    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![FollowEpsilon::Explore(StateID(0))];
    let mut curr_slots = vec![None; 1];
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: SlotTable::new(),
    };
    let haystack = b"c";
    let at = 0;
    let sid = StateID(0);

    pike_vm.epsilon_closure(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_at_end_of_haystack() {
    let nfa = NFA {
        pattern: String::from("d"),
        states: vec![State::Char { code: 'd' }, State::Match],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![None],
        memory_extra: 0,
    };

    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![FollowEpsilon::Explore(StateID(0))];
    let mut curr_slots = vec![None; 1];
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: SlotTable::new(),
    };
    let haystack = b"d";
    let at = 0;
    let sid = StateID(0);

    pike_vm.epsilon_closure(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

