// Answer 0

#[test]
fn test_next_with_fail_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Fail],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: Default::default(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![];
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: curr_slot_table.clone(),
    };
    let haystack = b"test";
    let at = 0;
    let at_ch = 't';
    let at_len = at_ch.len_utf8();
    let sid = StateID(0);

    pike_vm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_goto_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Goto { target: StateID(1), look: None }],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: Default::default(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![];
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: curr_slot_table.clone(),
    };
    let haystack = b"test";
    let at = 0;
    let at_ch = 't';
    let at_len = at_ch.len_utf8();
    let sid = StateID(0);

    pike_vm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_splits_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Splits { targets: vec![StateID(1)], reverse: false }],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: Default::default(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![];
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: curr_slot_table.clone(),
    };
    let haystack = b"test";
    let at = 0;
    let at_ch = 't';
    let at_len = at_ch.len_utf8();
    let sid = StateID(0);

    pike_vm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_capture_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Capture { target: StateID(1), slot: 0 }],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: Default::default(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut stack = vec![];
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: curr_slot_table.clone(),
    };
    let haystack = b"test";
    let at = 0;
    let at_ch = 't';
    let at_len = at_ch.len_utf8();
    let sid = StateID(0);

    pike_vm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

