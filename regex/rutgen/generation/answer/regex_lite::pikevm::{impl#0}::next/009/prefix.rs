// Answer 0

#[test]
fn test_next_with_ranges_equal_start() {
    let ranges = vec![(b'a' as char, b'b' as char)];
    let states = vec![State::Ranges { target: StateID(1), ranges }];
    let nfa = NFA {
        pattern: "a".to_string(),
        states,
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    
    let pike_vm = PikeVM::new(nfa.clone());
    let mut stack = vec![];
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: curr_slot_table.clone(),
    };
    
    let haystack = b"abcd"; 
    let at = 0; 
    let at_ch = 'a'; 
    let at_len = 1; 
    let sid = StateID(0);
    
    pike_vm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_ranges_equal_end() {
    let ranges = vec![(b'a' as char, b'a' as char)];
    let states = vec![State::Ranges { target: StateID(1), ranges }];
    let nfa = NFA {
        pattern: "a".to_string(),
        states,
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    
    let pike_vm = PikeVM::new(nfa.clone());
    let mut stack = vec![];
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: curr_slot_table.clone(),
    };
    
    let haystack = b"abcd"; 
    let at = 0; 
    let at_ch = 'a'; 
    let at_len = 1; 
    let sid = StateID(0);
    
    pike_vm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

#[test]
fn test_next_with_no_valid_ranges() {
    let ranges = vec![(b'a' as char, b'c' as char)];
    let states = vec![State::Ranges { target: StateID(1), ranges }];
    let nfa = NFA {
        pattern: "a".to_string(),
        states,
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: vec![],
        memory_extra: 0,
    };
    
    let pike_vm = PikeVM::new(nfa.clone());
    let mut stack = vec![];
    let mut curr_slot_table = SlotTable::new();
    let mut next = ActiveStates {
        set: SparseSet::new(),
        slot_table: curr_slot_table.clone(),
    };
    
    let haystack = b"abcd"; 
    let at = 0; 
    let at_ch = 'd'; 
    let at_len = 1; 
    let sid = StateID(0);
    
    pike_vm.next(&mut stack, &mut curr_slot_table, &mut next, haystack, at, at_ch, at_len, sid);
}

