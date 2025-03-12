// Answer 0

#[test]
fn test_state_sparse_single_transition() {
    let transitions = SparseTransitions {
        transitions: Box::from([Transition { start: 0u8, end: 1u8, next: StateID(SmallIndex::new_unchecked(1)) }]),
    };
    let state = State::Sparse(transitions);
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_sparse_multiple_transitions() {
    let transitions = SparseTransitions {
        transitions: Box::from([
            Transition { start: 0u8, end: 0u8, next: StateID(SmallIndex::new_unchecked(1)) },
            Transition { start: 2u8, end: 2u8, next: StateID(SmallIndex::new_unchecked(2)) },
            Transition { start: 3u8, end: 5u8, next: StateID(SmallIndex::new_unchecked(3)) },
        ]),
    };
    let state = State::Sparse(transitions);
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_sparse_empty_transitions() {
    let transitions = SparseTransitions {
        transitions: Box::from([]),
    };
    let state = State::Sparse(transitions);
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_sparse_large_index() {
    let transitions = SparseTransitions {
        transitions: Box::from([
            Transition { start: 0u8, end: 0u8, next: StateID(SmallIndex::new_unchecked(255)) },
            Transition { start: 1u8, end: 1u8, next: StateID(SmallIndex::new_unchecked(254)) },
        ]),
    };
    let state = State::Sparse(transitions);
    let _ = format!("{:?}", state);
}

