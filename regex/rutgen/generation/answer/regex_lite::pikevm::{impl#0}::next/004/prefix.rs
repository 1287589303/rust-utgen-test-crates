// Answer 0

#[test]
fn test_next_with_goto_state() {
    let nfa = NFA {
        pattern: String::from("a"),
        states: vec![State::Goto { target: StateID(1), look: None }],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pvm = PikeVM::new(nfa);
    let mut stack = Vec::new();
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates { set: SparseSet::new(), slot_table: curr_slot_table.clone() };
    let haystack = b"abc";
    let at = 0;
    let at_ch = 'a';
    let at_len = 1;
    let sid = StateID(0);
    
    pvm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_splits_state() {
    let nfa = NFA {
        pattern: String::from("b"),
        states: vec![State::Splits { targets: vec![StateID(1)], reverse: false }],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pvm = PikeVM::new(nfa);
    let mut stack = Vec::new();
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates { set: SparseSet::new(), slot_table: curr_slot_table.clone() };
    let haystack = b"xyz";
    let at = 1;
    let at_ch = 'y';
    let at_len = 1;
    let sid = StateID(0);
    
    pvm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_fail_state() {
    let nfa = NFA {
        pattern: String::from("c"),
        states: vec![State::Fail],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pvm = PikeVM::new(nfa);
    let mut stack = Vec::new();
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates { set: SparseSet::new(), slot_table: curr_slot_table.clone() };
    let haystack = b"def";
    let at = 2;
    let at_ch = 'f';
    let at_len = 1;
    let sid = StateID(0);
    
    pvm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_capture_state() {
    let nfa = NFA {
        pattern: String::from("d"),
        states: vec![State::Capture { target: StateID(1), slot: 0 }],
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    let pvm = PikeVM::new(nfa);
    let mut stack = Vec::new();
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates { set: SparseSet::new(), slot_table: curr_slot_table.clone() };
    let haystack = b"ghi";
    let at = 0;
    let at_ch = 'g';
    let at_len = 1;
    let sid = StateID(0);
    
    pvm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

