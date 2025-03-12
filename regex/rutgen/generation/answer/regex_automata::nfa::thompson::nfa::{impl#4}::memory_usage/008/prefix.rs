// Answer 0

#[test]
fn test_memory_usage_sparse_single_transition() {
    let transitions = SparseTransitions {
        transitions: Box::new([Transition { start: 0, end: 10, next: StateID(SmallIndex(1)) }]),
    };
    let state = State::Sparse(transitions);
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_sparse_multiple_transitions() {
    let transitions = SparseTransitions {
        transitions: Box::new([
            Transition { start: 0, end: 10, next: StateID(SmallIndex(1)) },
            Transition { start: 11, end: 20, next: StateID(SmallIndex(2)) },
            Transition { start: 21, end: 30, next: StateID(SmallIndex(3)) },
        ]),
    };
    let state = State::Sparse(transitions);
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_sparse_max_transitions() {
    let transitions = SparseTransitions {
        transitions: Box::new((0..256).map(|i| Transition { start: i as u8, end: (i + 10) as u8, next: StateID(SmallIndex(i as u32)) }).collect::<Vec<_>>().into_boxed_slice()),
    };
    let state = State::Sparse(transitions);
    let _ = state.memory_usage();
}

