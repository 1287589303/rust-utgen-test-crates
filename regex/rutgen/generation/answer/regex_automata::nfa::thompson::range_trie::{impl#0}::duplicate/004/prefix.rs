// Answer 0

#[test]
fn test_duplicate_with_valid_state() {
    let mut trie = RangeTrie::new();
    
    // Create a non-final state with transitions.
    let state_id = trie.add_empty();
    let transition = Transition {
        range: Utf8Range::from(1..=5),
        next_id: StateID::new_unchecked(2), // Non-final state ID
    };

    trie.add_transition(state_id, transition.range, transition.next_id);

    // Invoke the duplicate function.
    let new_id = trie.duplicate(state_id);
}

#[test]
fn test_duplicate_with_multiple_transitions() {
    let mut trie = RangeTrie::new();
    
    // Create a non-final state with multiple transitions.
    let state_id = trie.add_empty();
    let transition1 = Transition {
        range: Utf8Range::from(1..=5),
        next_id: StateID::new_unchecked(2), // Non-final state ID
    };
    let transition2 = Transition {
        range: Utf8Range::from(6..=10),
        next_id: StateID::new_unchecked(3), // Another non-final state ID
    };

    trie.add_transition(state_id, transition1.range, transition1.next_id);
    trie.add_transition(state_id, transition2.range, transition2.next_id);

    // Invoke the duplicate function.
    let new_id = trie.duplicate(state_id);
}

#[test]
fn test_duplicate_with_nesting() {
    let mut trie = RangeTrie::new();
    
    // Create a root state with transitions pointing to another non-final state.
    let state_id = trie.add_empty();
    let inner_state_id = trie.add_empty();

    let transition = Transition {
        range: Utf8Range::from(1..=5),
        next_id: inner_state_id, // Non-final state ID
    };

    trie.add_transition(state_id, transition.range, transition.next_id);

    // Add a transition to a final state to the inner state.
    let final_transition = Transition {
        range: Utf8Range::from(6..=10),
        next_id: FINAL, // Final state
    };
    
    trie.add_transition(inner_state_id, final_transition.range, final_transition.next_id);

    // Invoke the duplicate function.
    let new_id = trie.duplicate(state_id);
}

