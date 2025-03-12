// Answer 0

#[test]
fn test_is_epsilon_dense() {
    let state = State::Dense {
        transitions: Box::new([StateID::DEFAULT; 256]),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_sparse() {
    let state = State::Sparse {
        transitions: Box::new([
            Transition { start: 0, end: 255, next: StateID::ZERO },
        ]),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_byterange() {
    let state = State::ByteRange {
        trans: Transition { start: 0, end: 255, next: StateID::ZERO },
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_match() {
    let state = State::Match {
        pattern_id: PatternID::ZERO,
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_fail() {
    let state = State::Fail;
    state.is_epsilon();
}

