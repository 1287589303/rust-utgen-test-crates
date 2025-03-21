// Answer 0

#[test]
fn test_nexts_no_active_states() {
    let haystack = b"test input";
    let input = Input {
        haystack,
        span: Span::new(0, haystack.len()),
        anchored: Anchored::Yes,
        earliest: false,
    };
    let mut stack = Vec::new();
    
    let curr_set = SparseSet::new(0);
    let curr_slot_table = SlotTable {
        table: vec![None; 1],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let curr = ActiveStates {
        set: curr_set,
        slot_table: curr_slot_table,
    };

    let next_set = SparseSet::new(0);
    let next_slot_table = SlotTable {
        table: vec![None; 1],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let mut next = ActiveStates {
        set: next_set,
        slot_table: next_slot_table,
    };

    let slots = &mut [None];
    let pike_vm = PikeVM {
        config: Config::new(),
        nfa: NFA::default(),
    };

    let pid = pike_vm.nexts(&mut stack, &mut curr, &mut next, &input, 0, slots);
}

#[test]
fn test_nexts_empty_slots() {
    let haystack = b"input";
    let input = Input {
        haystack,
        span: Span::new(0, haystack.len()),
        anchored: Anchored::Yes,
        earliest: false,
    };
    let mut stack = Vec::new();
    
    let curr_set = SparseSet::new(0);
    let curr_slot_table = SlotTable {
        table: vec![None; 1],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let curr = ActiveStates {
        set: curr_set,
        slot_table: curr_slot_table,
    };

    let next_set = SparseSet::new(0);
    let next_slot_table = SlotTable {
        table: vec![None; 1],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let mut next = ActiveStates {
        set: next_set,
        slot_table: next_slot_table,
    };

    let slots = &mut [None];
    let pike_vm = PikeVM {
        config: Config::new(),
        nfa: NFA::default(),
    };

    let pid = pike_vm.nexts(&mut stack, &mut curr, &mut next, &input, 0, slots);
}

