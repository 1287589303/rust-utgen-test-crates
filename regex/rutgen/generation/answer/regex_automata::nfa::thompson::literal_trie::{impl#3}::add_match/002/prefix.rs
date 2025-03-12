// Answer 0

#[test]
fn test_add_match_with_empty_transitions_and_non_empty_chunks() {
    let mut state = State {
        transitions: Vec::new(), // empty transitions
        chunks: vec![(0, 1)], // non-empty chunks with one entry
    };
    state.add_match();
}

#[test]
fn test_add_match_with_empty_transitions_and_multiple_chunks() {
    let mut state = State {
        transitions: Vec::new(), // empty transitions
        chunks: vec![(0, 3), (3, 5)], // non-empty chunks with multiple entries
    };
    state.add_match();
}

