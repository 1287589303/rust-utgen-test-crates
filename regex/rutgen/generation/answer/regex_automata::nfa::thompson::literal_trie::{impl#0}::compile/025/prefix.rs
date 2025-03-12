// Answer 0

#[test]
fn test_compile_literal_trie() {
    let mut builder = Builder::new();
    let mut trie = LiteralTrie::forward();
    let empty_state_id = builder.add_empty().unwrap();

    // Construct a non-leaf state with a transition
    let transition_byte = 97; // 'a'
    let next_state_id = StateID::ZERO; // Using a placeholder state ID
    let transition = Transition {
        byte: transition_byte,
        next: next_state_id,
    };
    
    // Create a state that is not a leaf (has transitions)
    trie.states.push(State {
        transitions: vec![transition.clone()],
        chunks: vec![(0, 1)], // Simplified chunk representation
    });

    // Add a leaf state to keep the trie valid
    trie.states.push(State {
        transitions: vec![],
        chunks: vec![],
    });

    // Add the transition to ensure we have valid non-leaf states
    assert!(builder.add(State::ByteRange { trans: transition }).is_ok());

    // Set up Frame with non-leaf state
    let mut f = Frame::new(&trie.states[0]);
    f.sparse.push(thompson::Transition {
        start: transition.byte,
        end: transition.byte,
        next: empty_state_id,
    });

    // Now invoke compile
    let result = trie.compile(&mut builder);
    let expected = Ok(ThompsonRef {
        start: StateID::ZERO,
        end: empty_state_id,
    });
    assert_eq!(result, expected);
}

