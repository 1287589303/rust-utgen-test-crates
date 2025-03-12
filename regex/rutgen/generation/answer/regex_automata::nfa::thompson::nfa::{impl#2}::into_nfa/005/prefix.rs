// Answer 0

#[test]
fn test_into_nfa_with_binary_union() {
    let start_pattern = vec![StateID(Default::default())];
    let state_id = StateID(Default::default());
    
    let states = vec![
        State::BinaryUnion {
            alt1: StateID(Default::default()),
            alt2: StateID(Default::default()),
        },
        State::BinaryUnion {
            alt1: StateID(Default::default()),
            alt2: StateID(Default::default()),
        },
        State::Match { pattern_id: PatternID(Default::default()) },
    ];

    let mut inner = Inner {
        states,
        start_pattern,
        ..Default::default()
    };

    inner.into_nfa();
}

#[test]
fn test_into_nfa_with_other_states_before_binary_union() {
    let start_pattern = vec![StateID(Default::default())];
    
    let states = vec![
        State::ByteRange { trans: Default::default() },
        State::BinaryUnion {
            alt1: StateID(Default::default()),
            alt2: StateID(Default::default()),
        },
        State::Match { pattern_id: PatternID(Default::default()) },
    ];

    let mut inner = Inner {
        states,
        start_pattern,
        ..Default::default()
    };

    inner.into_nfa();
}

#[test]
fn test_into_nfa_with_multiple_binary_union_states() {
    let start_pattern = vec![StateID(Default::default())];

    let states = vec![
        State::BinaryUnion {
            alt1: StateID(Default::default()),
            alt2: StateID(Default::default()),
        },
        State::BinaryUnion {
            alt1: StateID(Default::default()),
            alt2: StateID(Default::default()),
        },
        State::Match { pattern_id: PatternID(Default::default()) },
    ];

    let mut inner = Inner {
        states,
        start_pattern,
        ..Default::default()
    };

    inner.into_nfa();
}

