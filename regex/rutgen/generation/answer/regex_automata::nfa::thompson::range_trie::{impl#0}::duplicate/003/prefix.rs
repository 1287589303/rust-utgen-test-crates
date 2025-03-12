// Answer 0

#[test]
fn test_duplicate_with_existing_transitions() {
    let mut trie = RangeTrie::new();
    
    // Add a state with valid transitions
    let old_id = trie.add_empty();
    let transition_range = Utf8Range::from(0..=127);
    let transition_next_id = StateID::new_unchecked(2); // not FINAL
    trie.add_transition(old_id, transition_range.clone(), transition_next_id);
    
    // Add a transition that leads to FINAL
    trie.add_transition(transition_next_id, Utf8Range::from(128..=255), FINAL);

    // Now we can call the duplicate function
    let new_id = trie.duplicate(old_id);
}

#[test]
fn test_duplicate_with_multiple_transitions_leading_to_final() {
    let mut trie = RangeTrie::new();

    // Create the first state
    let old_id = trie.add_empty();
    let transition1_next_id = StateID::new_unchecked(2); // not FINAL
    let transition2_next_id = StateID::new_unchecked(3); // not FINAL
    trie.add_transition(old_id, Utf8Range::from(0..=127), transition1_next_id);
    trie.add_transition(old_id, Utf8Range::from(128..=255), transition2_next_id);

    // Both transitions lead to FINAL
    trie.add_transition(transition1_next_id, Utf8Range::from(0..=255), FINAL);
    trie.add_transition(transition2_next_id, Utf8Range::from(0..=255), FINAL);

    // Call duplicate
    let new_id = trie.duplicate(old_id);
}

#[test]
fn test_duplicate_with_non_final_transition() {
    let mut trie = RangeTrie::new();

    // Add a state and several transitions
    let old_id = trie.add_empty();
    let transition_next_id1 = StateID::new_unchecked(2);
    let transition_next_id2 = StateID::new_unchecked(3); // not FINAL
    trie.add_transition(old_id, Utf8Range::from(0..=127), transition_next_id1);
    trie.add_transition(transition_next_id1, Utf8Range::from(128..=255), transition_next_id2);

    // Set transition that leads to FINAL for second state
    trie.add_transition(transition_next_id2, Utf8Range::from(0..=255), FINAL);

    // Duplicate the old state
    let new_id = trie.duplicate(old_id);
}

#[test]
fn test_duplicate_with_empty_transitions_stack() {
    let mut trie = RangeTrie::new();
    
    // Add states and ensure some transitions lead to FINAL
    let old_id = trie.add_empty();
    let transition_next_id = StateID::new_unchecked(2); // not FINAL
    trie.add_transition(old_id, Utf8Range::from(0..=127), transition_next_id);
    trie.add_transition(transition_next_id, Utf8Range::from(128..=255), FINAL);

    // Call duplicate
    let new_id = trie.duplicate(old_id);
}

