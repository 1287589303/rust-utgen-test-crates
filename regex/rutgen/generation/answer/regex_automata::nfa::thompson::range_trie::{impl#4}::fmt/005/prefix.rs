// Answer 0

#[test]
fn test_fmt_with_non_empty_states() {
    let state_id_one = StateID::new_unchecked(1);
    let state_id_two = StateID::new_unchecked(2);
    
    let states = vec![
        State::Match { pattern_id: 0 },  // State with match
        State::ByteRange { trans: Transition::new(state_id_one, 0..=127) }, // Non-final state
    ];

    let range_trie = RangeTrie {
        states,
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };

    let mut output = String::new();
    range_trie.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_with_multiple_states() {
    let state_id_one = StateID::new_unchecked(1);
    let state_id_two = StateID::new_unchecked(2);
    
    let states = vec![
        State::ByteRange { trans: Transition::new(state_id_one, 32..=64) }, // Non-final state
        State::Match { pattern_id: 1 },  // Match state
        State::ByteRange { trans: Transition::new(state_id_two, 128..=255) }, // Non-final state
    ];

    let range_trie = RangeTrie {
        states,
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };

    let mut output = String::new();
    range_trie.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_with_sparse_transitions() {
    let state_id_one = StateID::new_unchecked(1);

    let states = vec![
        State::Sparse { transitions: vec![Transition::new(state_id_one, 0..=127)] }, // Non-final state
        State::Match { pattern_id: 2 }, // Match state
    ];

    let range_trie = RangeTrie {
        states,
        free: vec![],
        iter_stack: RefCell::new(vec![]),
        iter_ranges: RefCell::new(vec![]),
        dupe_stack: vec![],
        insert_stack: vec![],
    };

    let mut output = String::new();
    range_trie.fmt(&mut output).unwrap();
}

