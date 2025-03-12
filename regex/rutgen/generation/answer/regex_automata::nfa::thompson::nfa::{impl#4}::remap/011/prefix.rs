// Answer 0

#[test]
fn test_remap_sparse_empty_transitions() {
    let mut state = State::Sparse(SparseTransitions {
        transitions: Box::new([]),
    });
    let remap: Vec<StateID> = vec![StateID(SmallIndex(1))]; // valid remap with at least one element
    state.remap(&remap);
}

#[test]
fn test_remap_sparse_non_empty_remap() {
    let mut state = State::Sparse(SparseTransitions {
        transitions: Box::new([]),
    });
    let remap: Vec<StateID> = vec![StateID(SmallIndex(1)), StateID(SmallIndex(2))]; // valid remap with more than one element
    state.remap(&remap);
}

