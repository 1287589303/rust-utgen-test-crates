// Answer 0

#[test]
fn test_remap_sparse_with_one_transition() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));

    let transition_1 = Transition {
        start: 0,
        end: 0,
        next: state_id_1,
    };

    let transitions = SparseTransitions {
        transitions: Box::new([transition_1]),
    };

    let mut state = State::Sparse(transitions);
    let remap = [StateID(SmallIndex(0))];

    state.remap(&remap);
}

#[test]
fn test_remap_sparse_with_multiple_transitions() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));

    let transition_1 = Transition {
        start: 0,
        end: 0,
        next: state_id_1,
    };
    let transition_2 = Transition {
        start: 1,
        end: 1,
        next: state_id_2,
    };
    let transition_3 = Transition {
        start: 2,
        end: 2,
        next: state_id_3,
    };

    let transitions = SparseTransitions {
        transitions: Box::new([transition_1, transition_2, transition_3]),
    };

    let mut state = State::Sparse(transitions);
    let remap = [
        StateID(SmallIndex(2)),
        StateID(SmallIndex(0)),
        StateID(SmallIndex(1)),
    ];

    state.remap(&remap);
}

#[test]
fn test_remap_sparse_with_no_transitions() {
    let transitions = SparseTransitions {
        transitions: Box::new([]),
    };

    let mut state = State::Sparse(transitions);
    let remap: Vec<StateID> = Vec::new();

    state.remap(&remap);
}

