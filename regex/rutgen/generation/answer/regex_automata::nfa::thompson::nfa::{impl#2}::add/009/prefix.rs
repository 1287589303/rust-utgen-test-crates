// Answer 0

#[test]
fn test_add_sparse_with_zero_transitions() {
    let mut inner = Inner::default();
    let sparse_transitions = SparseTransitions {
        transitions: Box::from([]), // Empty array to satisfy zero transitions
    };
    let state = State::Sparse(sparse_transitions);
    let _id = inner.add(state);
}

#[test]
fn test_add_multiple_sparse_with_zero_transitions() {
    let mut inner = Inner::default();

    let sparse_transitions_1 = SparseTransitions {
        transitions: Box::from([]),
    };
    let state_1 = State::Sparse(sparse_transitions_1);
    let _id_1 = inner.add(state_1);

    let sparse_transitions_2 = SparseTransitions {
        transitions: Box::from([]),
    };
    let state_2 = State::Sparse(sparse_transitions_2);
    let _id_2 = inner.add(state_2);
}

