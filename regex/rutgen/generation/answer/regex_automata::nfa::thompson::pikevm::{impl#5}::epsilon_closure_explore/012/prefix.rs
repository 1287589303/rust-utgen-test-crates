// Answer 0

#[test]
fn test_epsilon_closure_explore_with_dense_state() {
    let mut stack = Vec::new();
    let mut curr_slots = vec![None; 10];
    let mut next = ActiveStates {
        set: SparseSet::new(5),
        slot_table: SlotTable::new(),
    };
    let haystack: &[u8] = b"sample input";
    let input = Input::new(&haystack).set_span((0, haystack.len()));
    let sid = StateID(SmallIndex::new(2).unwrap());

    let pike_vm = PikeVM {
        config: Config { ..Default::default() },
        nfa: NFA::new("pattern").unwrap(),
    };

    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, &input, 0, sid);
}

#[test]
fn test_epsilon_closure_explore_with_sparse_state() {
    let mut stack = Vec::new();
    let mut curr_slots = vec![None; 10];
    let mut next = ActiveStates {
        set: SparseSet::new(5),
        slot_table: SlotTable::new(),
    };
    let haystack: &[u8] = b"another example";
    let input = Input::new(&haystack).set_span((0, haystack.len()));
    let sid = StateID(SmallIndex::new(3).unwrap());

    let pike_vm = PikeVM {
        config: Config { ..Default::default() },
        nfa: NFA::new("another_pattern").unwrap(),
    };

    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, &input, 0, sid);
}

#[test]
fn test_epsilon_closure_explore_with_byte_range_state() {
    let mut stack = Vec::new();
    let mut curr_slots = vec![None; 10];
    let mut next = ActiveStates {
        set: SparseSet::new(5),
        slot_table: SlotTable::new(),
    };
    let haystack: &[u8] = b"third test case";
    let input = Input::new(&haystack).set_span((0, haystack.len()));
    let sid = StateID(SmallIndex::new(1).unwrap());

    let pike_vm = PikeVM {
        config: Config { ..Default::default() },
        nfa: NFA::new("test_pattern").unwrap(),
    };

    pike_vm.epsilon_closure_explore(&mut stack, &mut curr_slots, &mut next, &input, 0, sid);
}

