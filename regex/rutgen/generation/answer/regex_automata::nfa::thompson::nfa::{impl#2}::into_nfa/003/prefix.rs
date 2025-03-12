// Answer 0

#[test]
fn test_into_nfa_with_fail_state() {
    let mut inner = Inner {
        states: vec![
            State::Fail,
            State::ByteRange { trans: Transition::default() },
        ],
        start_pattern: vec![StateID::default()],
        ..Default::default()
    };

    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_byte_range_state() {
    let mut inner = Inner {
        states: vec![
            State::ByteRange { trans: Transition::default() },
            State::Fail,
        ],
        start_pattern: vec![StateID::default()],
        ..Default::default()
    };

    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_dense_state() {
    let mut inner = Inner {
        states: vec![
            State::Dense { transitions: vec![] },
            State::Fail,
        ],
        start_pattern: vec![StateID::default()],
        ..Default::default()
    };

    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_multiple_states() {
    let mut inner = Inner {
        states: vec![
            State::ByteRange { trans: Transition::default() },
            State::Dense { transitions: vec![] },
            State::Fail,
        ],
        start_pattern: vec![StateID::default(), StateID::default()],
        ..Default::default()
    };

    let nfa = inner.into_nfa();
}

