// Answer 0

#[test]
fn test_epsilon_closure_explore_with_state_goto_none() {
    let haystack: Vec<u8> = vec![b'a'];
    let mut stack = vec![];
    let mut curr_slots = vec![None; 2];
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };
    let nfa = NFA {
        states: vec![
            State::Goto { target: StateID(1), look: None },
            State::Match,
        ],
        ..Default::default()
    };
    let pike_vm = PikeVM { nfa };

    let sid = StateID(0);
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, &haystack, 0, sid);
}

#[test]
fn test_epsilon_closure_explore_with_state_goto_some_and_match() {
    let haystack: Vec<u8> = vec![b'a'];
    let mut stack = vec![];
    let mut curr_slots = vec![None; 2];
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };
    let nfa = NFA {
        states: vec![
            State::Goto { target: StateID(1), look: Some(Look::Start) },
            State::Match,
        ],
        ..Default::default()
    };
    let pike_vm = PikeVM { nfa };

    let sid = StateID(0);
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, &haystack, 0, sid);
}

#[test]
fn test_epsilon_closure_explore_with_state_ranges() {
    let haystack: Vec<u8> = vec![b'a'];
    let mut stack = vec![];
    let mut curr_slots = vec![None; 2];
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };
    let nfa = NFA {
        states: vec![
            State::Ranges { target: StateID(1), ranges: vec![('a', 'z')] },
            State::Match,
        ],
        ..Default::default()
    };
    let pike_vm = PikeVM { nfa };

    let sid = StateID(0);
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, &haystack, 0, sid);
}

#[test]
fn test_epsilon_closure_explore_with_insert_duplicate_state() {
    let haystack: Vec<u8> = vec![b'a'];
    let mut stack = vec![];
    let mut curr_slots = vec![None; 2];
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::new(),
    };
    let nfa = NFA {
        states: vec![
            State::Goto { target: StateID(1), look: Some(Look::Word) },
            State::Goto { target: StateID(0), look: None },
        ],
        ..Default::default()
    };
    let pike_vm = PikeVM { nfa };

    let sid = StateID(0);
    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, &haystack, 0, sid);
}

