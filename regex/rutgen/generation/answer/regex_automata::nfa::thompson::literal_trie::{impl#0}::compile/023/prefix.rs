// Answer 0

#[test]
fn test_compile_with_single_sparse_transition_failure() {
    let mut builder = Builder::new();
    let mut trie = LiteralTrie {
        states: vec![State {
            transitions: vec![Transition { byte: b'a', next: StateID::from(1) }],
        }, State {
            transitions: vec![],
        }],
        rev: false,
    };

    // Simulate adding a single state with a transition that will eventually fail
    let _ = builder.add_empty().unwrap();
    let result = trie.compile(&mut builder);
    // Call to verify compilation, expecting an error due to single sparse
    // transition with a builder that cannot accept the sparse transition.
    let _ = result.unwrap_err();
}

#[test]
fn test_compile_with_failed_add_sparse_due_to_configuration() {
    let mut builder = Builder::new();
    let mut trie = LiteralTrie {
        states: vec![State {
            transitions: vec![Transition { byte: b'a', next: StateID::from(1) }],
            chunks: vec![(0, 1)],
        }, State {
            transitions: vec![],
            chunks: vec![],
        }],
        rev: false,
    };

    // Ensure add_empty() returns Ok/Some
    let _ = builder.add_empty().unwrap();
    let result = trie.compile(&mut builder);
    // Expecting an error from the builder when trying to add sparse transitions
    let _ = result.unwrap_err();
}

#[test]
fn test_compile_with_leading_transition_for_sparse_failure() {
    let mut builder = Builder::new();
    let mut trie = LiteralTrie {
        states: vec![State {
            transitions: vec![Transition { byte: b'a', next: StateID::from(2) }],
            chunks: vec![(0, 1)],
        }, State {
            transitions: vec![
                Transition { byte: b'b', next: StateID::from(1) },
                Transition { byte: b'c', next: StateID::from(3) },
            ],
            chunks: vec![],
        }, State {
            transitions: vec![],
            chunks: vec![],
        }],
        rev: false,
    };

    // Ensure add_empty() returns Ok/Some
    let _ = builder.add_empty().unwrap();
    // Force a transition to be active and lead to a configuration that
    // does not allow adding sparse.
    let result = trie.compile(&mut builder);
    // Expecting an error from the builder when trying to add sparsely.
    let _ = result.unwrap_err();
}

