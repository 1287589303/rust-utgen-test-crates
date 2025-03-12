// Answer 0

#[test]
fn test_validate_all_conditions_met() {
    #[derive(Clone)]
    struct TestTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    let mut sparse_data = vec![0u8; 256];
    let classes = ByteClasses([0; 256]);
    let transitions = TestTransitions {
        sparse: sparse_data,
        classes,
        state_len: 1,
        pattern_len: 1,
    };

    let special = Special {
        max: StateID(1),
        quit_id: StateID(0),
        min_match: StateID(1),
        max_match: StateID(1),
        min_accel: StateID(1),
        max_accel: StateID(1),
        min_start: StateID(1),
        max_start: StateID(1),
    };

    let mut id = StateID(0);
    while id.0 < transitions.sparse.len() {
        let state = transitions.try_state(&special, id).unwrap();
        let next_id = wire::add(id.0, state.write_to_len(), "next state ID offset").unwrap();
        id = StateID::new(next_id).unwrap();
    }

    assert_eq!(transitions.state_len, 1);
}

#[test]
fn test_validate_boundary_conditions() {
    #[derive(Clone)]
    struct TestTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    let sparse_data = vec![0u8; 256];
    let classes = ByteClasses([0; 256]);
    let transitions = TestTransitions {
        sparse: sparse_data,
        classes,
        state_len: 2,
        pattern_len: 1,
    };

    let special = Special {
        max: StateID(1),
        quit_id: StateID(0),
        min_match: StateID(1),
        max_match: StateID(1),
        min_accel: StateID(1),
        max_accel: StateID(1),
        min_start: StateID(1),
        max_start: StateID(1),
    };

    let id = StateID(1);
    let state = transitions.try_state(&special, id).unwrap();
    let next_id = wire::add(id.0, state.write_to_len(), "next state ID offset").unwrap();
    let new_id = StateID::new(next_id).unwrap();

    assert!(id.0 < transitions.sparse.len());
}

