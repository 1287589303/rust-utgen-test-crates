// Answer 0

#[test]
fn test_into_nfa_with_sparse_state() {
    let mut inner = Inner::default();
    let state_id: StateID = SmallIndex::new(0).into();
    inner.start_pattern.push(state_id);

    let sparse_transitions = vec![]; // Populate with appropriate transitions.
    inner.states.push(State::Sparse { transitions: sparse_transitions });

    inner.start_anchored = state_id;
    inner.start_unanchored = state_id;

    let nfa = inner.into_nfa(); // This should successfully execute.

    let state_id_1: StateID = SmallIndex::new(1).into();
    inner.start_pattern.push(state_id_1);

    // Add more states to ensure coverage
    inner.states.push(State::Sparse { transitions: sparse_transitions });
    let nfa_1 = inner.into_nfa(); // This should also execute without issues.

    let state_id_2: StateID = SmallIndex::new(2).into();
    inner.start_pattern.push(state_id_2);

    // More Sparse states.
    inner.states.push(State::Sparse { transitions: sparse_transitions });
    let nfa_2 = inner.into_nfa(); // Valid execution for multiple Sparse states.

    // Finally ensure some transition in the stack.
    let current_state = inner.states.len();
    inner.states.push(State::Sparse { transitions: vec![] });
    inner.start_pattern.push(StateID::new_unchecked(current_state as u32));
    let nfa_final = inner.into_nfa(); // This too should execute.
}

