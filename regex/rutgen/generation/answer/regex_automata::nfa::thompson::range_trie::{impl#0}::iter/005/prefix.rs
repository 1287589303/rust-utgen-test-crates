// Answer 0

#[test]
fn test_iter_non_final_state_with_transitions() {
    let mut trie = RangeTrie::new();
    let state_id = trie.add_empty(); // Assuming this adds a non-final state
    let transition = Transition {
        range: Utf8Range::new(1, 2), // Valid range
        next_id: StateID::new_unchecked(2), // Non-final ID
    };
    trie.add_transition(state_id, transition.range.clone(), transition.next_id);

    let result = trie.iter(|_| Ok(()));
    // result should equal Ok(())
}

#[test]
fn test_iter_with_tidx_leq_transitions_len() {
    let mut trie = RangeTrie::new();
    let state_id = trie.add_empty(); 
    let transition = Transition {
        range: Utf8Range::new(1, 3), 
        next_id: StateID::new_unchecked(3), // Non-final ID
    };
    trie.add_transition(state_id, transition.range.clone(), transition.next_id);
    let transition2 = Transition {
        range: Utf8Range::new(4, 5),
        next_id: StateID::FINAL,
    };
    trie.add_transition(state_id, transition2.range.clone(), transition2.next_id);

    let result = trie.iter(|_| Ok(()));
    // result should equal Ok(())
}

#[test]
fn test_iter_tidx_equals_transitions_len() {
    let mut trie = RangeTrie::new();
    let state_id = trie.add_empty(); 
    let transition = Transition {
        range: Utf8Range::new(1, 2), 
        next_id: StateID::new_unchecked(3),
    };
    trie.add_transition(state_id, transition.range.clone(), transition.next_id);
    let transition2 = Transition {
        range: Utf8Range::new(3, 4), 
        next_id: StateID::FINAL,
    };
    trie.add_transition(state_id, transition2.range.clone(), transition2.next_id);

    let result = trie.iter(|_| Ok(()));
    // result should equal Ok(())
}

#[test]
fn test_iter_stack_pop_valid_state() {
    let mut trie = RangeTrie::new();
    let state_id = trie.add_empty(); 
    let transition = Transition {
        range: Utf8Range::new(0, 1),
        next_id: StateID::new_unchecked(4), // Non-final ID
    };
    trie.add_transition(state_id, transition.range.clone(), transition.next_id);

    let result = trie.iter(|_| Ok(()));
    // result should equal Ok(())
}

