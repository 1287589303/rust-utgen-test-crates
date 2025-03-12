// Answer 0

#[test]
fn test_is_epsilon_with_sparse_transitions() {
    let state = State::Sparse {
        transitions: Box::new([]),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_dense_transitions() {
    let state = State::Dense {
        transitions: Box::new([StateID::ZERO; 256]),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_byte_range_transition() {
    let state = State::ByteRange {
        trans: Transition {
            start: 0,
            end: 255,
            next: StateID::ZERO,
        },
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_match_state() {
    let state = State::Match {
        pattern_id: PatternID::ZERO,
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_with_fail_state() {
    let state = State::Fail;
    state.is_epsilon();
}

