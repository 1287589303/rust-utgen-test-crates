// Answer 0

#[test]
fn test_is_epsilon_fail() {
    let state = State::Fail;
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_dense() {
    let transitions = Box::new([StateID::ZERO; 256]);
    let state = State::Dense(DenseTransitions { transitions });
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_sparse() {
    let transitions = Box::new([Transition { start: 0, end: 255, next: StateID::ZERO }]);
    let state = State::Sparse(SparseTransitions { transitions });
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_match() {
    let state = State::Match { pattern_id: PatternID::ZERO };
    state.is_epsilon();
}

