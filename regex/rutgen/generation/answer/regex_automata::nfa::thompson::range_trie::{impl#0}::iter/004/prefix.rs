// Answer 0

#[test]
fn test_iter_with_final_transition() {
    let mut trie = RangeTrie::new();
    let state_id = ROOT;

    // Creating a transition that leads to the FINAL state
    let transition = Transition {
        range: Utf8Range { start: 1, end: 3 },
        next_id: FINAL,
    };

    // Adding a state to the trie with the transition
    let state = State {
        transitions: vec![transition],
    };
    trie.states.push(state);

    // A function that processes the ranges
    let result = trie.iter(|ranges| {
        // Here we can just return Ok as the test is checking input handling
        Ok(())
    });

    // Verifying result is Ok
    assert!(result.is_ok());
}

#[test]
fn test_iter_no_more_transitions() {
    let mut trie = RangeTrie::new();
    let state_id = ROOT;

    // Adding a dummy state with no transitions
    let state = State {
        transitions: Vec::new(),
    };
    trie.states.push(state);

    // A function that processes the ranges
    let result = trie.iter(|ranges| {
        // Here we can just return Ok as the test is checking input handling
        Ok(())
    });

    // Verifying result is Ok
    assert!(result.is_ok());
}

#[test]
fn test_iter_with_multiple_transitions() {
    let mut trie = RangeTrie::new();
    let state_id = ROOT;

    // Creating transitions, one of which leads to FINAL
    let transition1 = Transition {
        range: Utf8Range { start: 1, end: 3 },
        next_id: StateID::new_unchecked(2), // An arbitrary new state
    };
    let transition2 = Transition {
        range: Utf8Range { start: 4, end: 6 },
        next_id: FINAL,
    };

    // Adding states with transitions
    let state = State {
        transitions: vec![transition1, transition2],
    };
    trie.states.push(state);
    
    // Adding another state for the transition
    let next_state = State {
        transitions: Vec::new(),
    };
    trie.states.push(next_state);

    // A function that processes the ranges
    let result = trie.iter(|ranges| {
        // Here we can just return Ok as the test is checking input handling
        Ok(())
    });

    // Verifying result is Ok
    assert!(result.is_ok());
}

#[test]
fn test_iter_single_transition_to_final() {
    let mut trie = RangeTrie::new();
    let state_id = ROOT;

    // Creating a single transition that leads to FINAL
    let transition = Transition {
        range: Utf8Range { start: 1, end: 1 },
        next_id: FINAL,
    };

    // Adding a state to the trie with the transition
    let state = State {
        transitions: vec![transition],
    };
    trie.states.push(state);

    // A function that processes the ranges
    let result = trie.iter(|ranges| {
        // Check that ranges contains the correct value
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], Utf8Range { start: 1, end: 1 });
        Ok(())
    });

    // Verifying result is Ok
    assert!(result.is_ok());
}

