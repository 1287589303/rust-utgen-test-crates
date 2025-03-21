// Answer 0

#[test]
fn test_next_ranges_no_matching_char() {
    let state_id = StateID(0);
    let target_state_id = StateID(1);
    let ranges = vec![(b'a' as char, b'z' as char)];
    let nfa = NFA {
        pattern: "test".to_string(),
        states: vec![
            State::Ranges { target: target_state_id, ranges },
            State::Match,
        ],
        start: state_id,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::default(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    let mut pike_vm = PikeVM::new(nfa);
    
    let haystack: &[u8] = b"1234567890"; // Any character not in the Ranges
    let at = 0;
    let at_ch = '1'; // Not in the range of 'a' to 'z'
    let at_len = 1;
    
    let mut stack = Vec::new();
    let mut active_states = ActiveStates {
        set: SparseSet::new(),
        slot_table: SlotTable::new(),
    };
    let mut curr_slot_table = SlotTable::new();

    let result = pike_vm.next(&mut stack, &mut curr_slot_table, &mut active_states, haystack, at, at_ch, at_len, state_id);
}

#[test]
fn test_next_ranges_no_matching_char_with_longer_haystack() {
    let state_id = StateID(0);
    let target_state_id = StateID(1);
    let ranges = vec![(b'a' as char, b'z' as char)];
    let nfa = NFA {
        pattern: "test".to_string(),
        states: vec![
            State::Ranges { target: target_state_id, ranges },
            State::Match,
        ],
        start: state_id,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::default(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };

    let mut pike_vm = PikeVM::new(nfa);

    let haystack: &[u8] = b"Hello World!"; // Any character not in the Ranges
    let at = 0;
    let at_ch = 'H'; // Not in the range of 'a' to 'z'
    let at_len = 1;

    let mut stack = Vec::new();
    let mut active_states = ActiveStates {
        set: SparseSet::new(),
        slot_table: SlotTable::new(),
    };
    let mut curr_slot_table = SlotTable::new();

    let result = pike_vm.next(&mut stack, &mut curr_slot_table, &mut active_states, haystack, at, at_ch, at_len, state_id);
}

