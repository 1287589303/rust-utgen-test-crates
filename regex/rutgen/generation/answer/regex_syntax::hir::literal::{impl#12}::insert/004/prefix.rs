// Answer 0

#[test]
fn test_insert_with_existing_match() {
    let mut trie = PreferenceTrie {
        states: vec![State {
            trans: vec![(0, 1)], // Transition for byte 0 to state 1
        }],
        matches: vec![Some(NonZeroUsize::new(1).unwrap()), None], // Match for state 0
        next_literal_index: 2, // Next index to assign
    };
    let result = trie.insert(&[0]);
}

#[test]
fn test_insert_with_no_matching_state() {
    let mut trie = PreferenceTrie {
        states: vec![State::default()],
        matches: vec![None], // No matches
        next_literal_index: 1,
    };
    let result = trie.insert(&[1]); // Should create a new state
}

#[test]
fn test_insert_with_prefix_existing() {
    let mut trie = PreferenceTrie {
        states: vec![
            State {
                trans: vec![(0, 1)], // Transition for byte 0 to state 1
            },
            State {
                trans: vec![(1, 2)], // Transition for byte 1 to state 2
            },
        ],
        matches: vec![Some(NonZeroUsize::new(1).unwrap()), Some(NonZeroUsize::new(2).unwrap()), None],
        next_literal_index: 3, // Next index to assign
    };
    let result = trie.insert(&[0, 1]); // Should return an Err with the existing match index
}

