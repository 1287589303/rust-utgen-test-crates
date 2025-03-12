// Answer 0

#[test]
fn test_states_empty() {
    let nfa = NFA(Arc::new(Inner::default()));
    let states = nfa.states();
    assert!(states.is_empty());
}

#[test]
fn test_states_one_of_each_type() {
    let nfa = NFA(Arc::new(Inner {
        states: vec![
            State::ByteRange { trans: Transition::new() },
            State::Sparse(SparseTransitions::new()),
            State::Dense(DenseTransitions::new()),
            State::Union { alternates: Box::new([0, 1]) },
            State::Capture { next: 0, pattern_id: 0, group_index: 0.into(), slot: 0.into() },
            State::Match { pattern_id: 0 },
            State::Fail,
        ],
        ..Default::default()
    }));
    let states = nfa.states();
    assert_eq!(states.len(), 7);
}

#[test]
fn test_states_retrieval_by_id() {
    let mut inner = Inner::default();
    inner.states.extend(vec![
        State::ByteRange { trans: Transition::new() },
        State::Match { pattern_id: 0 },
    ]);
    let nfa = NFA(Arc::new(inner));
    let states = nfa.states();
    for id in 0..states.len() {
        let _state = nfa.state(id);
    }
}

#[test]
fn test_states_boundary_max() {
    let mut states = Vec::with_capacity(std::usize::MAX);
    for _ in 0..std::usize::MAX {
        states.push(State::ByteRange { trans: Transition::new() });
    }
    let nfa = NFA(Arc::new(Inner { states, ..Default::default() }));
    let retrieved_states = nfa.states();
    assert_eq!(retrieved_states.len(), std::usize::MAX);
}

#[test]
fn test_states_unique_ids() {
    let states = vec![
        State::ByteRange { trans: Transition::new() },
        State::Match { pattern_id: 0 },
    ];
    let nfa = NFA(Arc::new(Inner { states, ..Default::default() }));
    for id in 0..nfa.states().len() {
        assert_eq!(nfa.state(id).id(), id);
    }
}

#[test]
fn test_states_stability() {
    let nfa = NFA(Arc::new(Inner::default()));
    let states1 = nfa.states();
    let states2 = nfa.states();
    assert_eq!(states1, states2);
}

#[test]
fn test_states_null_references() {
    let nfa = NFA(Arc::new(Inner { states: Vec::new(), ..Default::default() }));
    assert!(nfa.states().is_empty());
}

