// Answer 0

#[test]
fn test_add_state_sparse_with_multiple_transitions() {
    let mut inner = Inner::default();
    let transitions = vec![
        Transition { start: 1, end: 5, next: StateID::default() },
        Transition { start: 6, end: 10, next: StateID::default() },
    ];
    let sparse = SparseTransitions { transitions: transitions.into_boxed_slice() };
    let state = State::Sparse(sparse);
    let id = inner.add(state);
}

#[test]
fn test_add_state_sparse_with_single_transition() {
    let mut inner = Inner::default();
    let transitions = vec![
        Transition { start: 0, end: 1, next: StateID::default() },
    ];
    let sparse = SparseTransitions { transitions: transitions.into_boxed_slice() };
    let state = State::Sparse(sparse);
    let id = inner.add(state);
}

#[test]
#[should_panic] // Expecting panic due to maximum state limit reached
fn test_add_state_sparse_exceeding_transition_limit() {
    let mut inner = Inner::default();
    let transitions: Vec<Transition> = (0..256)
        .map(|i| Transition { start: i as u8, end: i as u8, next: StateID::default() })
        .collect();
    let sparse = SparseTransitions { transitions: transitions.into_boxed_slice() };
    let state = State::Sparse(sparse);
    let id = inner.add(state);
}

#[test]
fn test_add_blank_state_sparse_no_transitions() {
    let mut inner = Inner::default();
    let sparse = SparseTransitions { transitions: Box::new([]) };
    let state = State::Sparse(sparse);
    let id = inner.add(state);
}

