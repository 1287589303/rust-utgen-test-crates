// Answer 0

#[test]
fn test_sparse_transitions_with_non_empty_transitions() {
    struct TestState<'a> {
        id: StateID,
        stride2: usize,
        transitions: &'a [StateID],
    }

    let state_id = StateID(SmallIndex::new(1));
    let transitions = &[StateID(SmallIndex::new(2)), StateID(SmallIndex::new(3))];
    let state = TestState {
        id: state_id,
        stride2: 2,
        transitions,
    };

    let _result = state.sparse_transitions();
}

#[test]
fn test_sparse_transitions_with_dead_transition() {
    struct TestState<'a> {
        id: StateID,
        stride2: usize,
        transitions: &'a [StateID],
    }

    let state_id = StateID(SmallIndex::new(0));
    let transitions = &[
        StateID(SmallIndex::new(1)), // Non-dead transition
        DEAD,                         // Dead transition
        StateID(SmallIndex::new(3)),
    ];
    let state = TestState {
        id: state_id,
        stride2: 3,
        transitions,
    };

    let _result = state.sparse_transitions();
}

#[test]
fn test_sparse_transitions_with_single_transition() {
    struct TestState<'a> {
        id: StateID,
        stride2: usize,
        transitions: &'a [StateID],
    }

    let state_id = StateID(SmallIndex::new(2));
    let transitions = &[StateID(SmallIndex::new(3))]; // Single non-dead transition
    let state = TestState {
        id: state_id,
        stride2: 1,
        transitions,
    };

    let _result = state.sparse_transitions();
}

#[test]
fn test_sparse_transitions_with_no_transitions() {
    struct TestState<'a> {
        id: StateID,
        stride2: usize,
        transitions: &'a [StateID],
    }

    let state_id = StateID(SmallIndex::new(2));
    let transitions: Vec<StateID> = vec![]; // No transitions
    let state = TestState {
        id: state_id,
        stride2: 0,
        transitions: transitions.as_slice(),
    };

    let _result = state.sparse_transitions();
}

