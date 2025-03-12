// Answer 0

#[test]
fn test_is_epsilon_with_match_state() {
    let state = State::Match {
        pattern_id: PatternID(SmallIndex::default()),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_dense_state() {
    let state = State::Dense(DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::default()); 256]),
    });
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_sparse_state() {
    let state = State::Sparse(SparseTransitions {
        transitions: Box::new([]),
    });
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_byte_range_state() {
    let state = State::ByteRange {
        trans: Transition { start: b'a', end: b'z', next: StateID(SmallIndex::default()) },
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_fail_state() {
    let state = State::Fail;
    state.is_epsilon();
}

