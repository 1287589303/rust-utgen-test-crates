// Answer 0

#[test]
fn test_next_for_ranges_state_with_start_equal_at_ch() {
    let target_id = StateID(1);
    let ranges = vec![(b'a' as char, b'a' as char)]; // start == at_ch
    let states = vec![State::Ranges { target: target_id, ranges }];
    let nfa = NFA {
        pattern: String::new(),
        states,
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::default(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut stack = Vec::new();
    let mut slot_table = SlotTable::new();
    let mut active_states = ActiveStates {
        set: SparseSet::default(),
        slot_table: slot_table.clone(),
    };
    let haystack: &[u8] = b"a"; // non-empty
    let at = 0; // valid index
    let at_ch = b'a' as char; // equal to start
    let at_len = 1; // greater than 0
    let sid = StateID(0); // valid for ranges

    let result = pike_vm.next(
        &mut stack,
        &mut slot_table,
        &mut active_states,
        haystack,
        at,
        at_ch,
        at_len,
        sid,
    );
}

#[test]
fn test_next_with_multiple_ranges_and_start_equal_at_ch() {
    let target_id = StateID(1);
    let ranges = vec![(b'a' as char, b'c' as char), (b'b' as char, b'b' as char)]; // start == at_ch
    let states = vec![State::Ranges { target: target_id, ranges }];
    let nfa = NFA {
        pattern: String::new(),
        states,
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::default(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut stack = Vec::new();
    let mut slot_table = SlotTable::new();
    let mut active_states = ActiveStates {
        set: SparseSet::default(),
        slot_table: slot_table.clone(),
    };
    let haystack: &[u8] = b"b"; // non-empty
    let at = 0; // valid index
    let at_ch = b'b' as char; // equal to start
    let at_len = 1; // greater than 0
    let sid = StateID(0); // valid for ranges

    let result = pike_vm.next(
        &mut stack,
        &mut slot_table,
        &mut active_states,
        haystack,
        at,
        at_ch,
        at_len,
        sid,
    );
}

#[test]
fn test_next_with_ranges_start_equal_at_ch_and_end_greater() {
    let target_id = StateID(1);
    let ranges = vec![(b'a' as char, b'd' as char)]; // start == at_ch and end >= at_ch
    let states = vec![State::Ranges { target: target_id, ranges }];
    let nfa = NFA {
        pattern: String::new(),
        states,
        start: StateID(0),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::default(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    let pike_vm = PikeVM::new(nfa);
    let mut stack = Vec::new();
    let mut slot_table = SlotTable::new();
    let mut active_states = ActiveStates {
        set: SparseSet::default(),
        slot_table: slot_table.clone(),
    };
    let haystack: &[u8] = b"a"; // non-empty
    let at = 0; // valid index
    let at_ch = b'a' as char; // equal to start
    let at_len = 1; // greater than 0
    let sid = StateID(0); // valid for ranges

    let result = pike_vm.next(
        &mut stack,
        &mut slot_table,
        &mut active_states,
        haystack,
        at,
        at_ch,
        at_len,
        sid,
    );
}

