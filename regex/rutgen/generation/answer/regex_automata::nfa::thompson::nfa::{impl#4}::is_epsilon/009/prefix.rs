// Answer 0

#[test]
fn test_is_epsilon_byte_range() {
    let state = State::ByteRange {
        trans: Transition { start: b'a', end: b'z', next: StateID::DEFAULT },
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_dense() {
    let state = State::Dense(DenseTransitions {
        transitions: Box::from([StateID::DEFAULT; 256]),
    });
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_sparse() {
    let state = State::Sparse(SparseTransitions {
        transitions: Box::new([]),
    });
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_match() {
    let state = State::Match {
        pattern_id: PatternID::DEFAULT,
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_fail() {
    let state = State::Fail;
    state.is_epsilon();
}

