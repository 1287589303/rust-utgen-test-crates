// Answer 0

#[test]
fn test_epsilon_closure_explore_with_valid_state_match() {
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![State::Match, State::Fail, State::Char { target: 1, ch: 'a' }],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: HashMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    let pike_vm = PikeVM { nfa };
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };

    let haystack: &[u8] = b"some test input";
    let at = 5;
    let sid = StateID(0);
    let mut curr_slots = vec![Some(NonMaxUsize::new(0).unwrap())];

    let mut stack = Vec::new();
    
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_explore_with_valid_state_ranges() {
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![
            State::Ranges { target: StateID(1), ranges: vec![('a', 'z')] },
            State::Char { target: StateID(2), ch: 'b' },
            State::Fail,
        ],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: HashMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    let pike_vm = PikeVM { nfa };
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };

    let haystack: &[u8] = b"some test input";
    let at = 5;
    let sid = StateID(0);
    let mut curr_slots = vec![Some(NonMaxUsize::new(1).unwrap()), None];

    let mut stack = Vec::new();
    
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_explore_with_valid_state_char() {
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![
            State::Char { target: StateID(1), ch: 'a' },
            State::Match,
            State::Fail,
        ],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: HashMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    let pike_vm = PikeVM { nfa };
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };

    let haystack: &[u8] = b"some test input";
    let at = 5;
    let sid = StateID(0);
    let mut curr_slots = vec![Some(NonMaxUsize::new(1).unwrap()), None];

    let mut stack = Vec::new();
    
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

#[test]
fn test_epsilon_closure_explore_with_valid_state_fail() {
    let nfa = NFA {
        pattern: String::from("test"),
        states: vec![
            State::Fail,
            State::Char { target: StateID(1), ch: 'b' },
            State::Match,
        ],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: HashMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    let pike_vm = PikeVM { nfa };
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };

    let haystack: &[u8] = b"some test input";
    let at = 5;
    let sid = StateID(0);
    let mut curr_slots = vec![Some(NonMaxUsize::new(2).unwrap()), None];

    let mut stack = Vec::new();
    
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, haystack, at, sid);
}

