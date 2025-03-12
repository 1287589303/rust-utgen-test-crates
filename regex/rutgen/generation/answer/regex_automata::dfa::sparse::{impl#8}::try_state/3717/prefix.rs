// Answer 0

#[test]
fn test_try_state_invalid_id_usize() {
    let id = StateID(257);
    let sparse_data = vec![0; 257];
    let transitions = Transitions {
        sparse: sparse_data,
        classes: ByteClasses([0; 256]),
        state_len: 257,
        pattern_len: 1,
    };
    let special = Special {
        max: StateID(256),
        quit_id: StateID(0),
        min_match: StateID(0),
        max_match: StateID(0),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };
    
    let _ = transitions.try_state(&special, id);
}

#[test]
fn test_try_state_invalid_transition_length() {
    let id = StateID(256);
    let sparse_data = vec![0; 260];
    let transitions = Transitions {
        sparse: sparse_data,
        classes: ByteClasses([0; 256]),
        state_len: 257,
        pattern_len: 1,
    };
    let special = Special {
        max: StateID(256),
        quit_id: StateID(0),
        min_match: StateID(0),
        max_match: StateID(0),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };

    let _ = transitions.try_state(&special, id);
}

#[test]
fn test_try_state_no_pattern_ids() {
    let id = StateID(256);
    let sparse_data = vec![0; 270];
    let transitions = Transitions {
        sparse: sparse_data,
        classes: ByteClasses([0; 256]),
        state_len: 257,
        pattern_len: 0,
    };
    let special = Special {
        max: StateID(256),
        quit_id: StateID(0),
        min_match: StateID(0),
        max_match: StateID(0),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };

    let _ = transitions.try_state(&special, id);
}

#[test]
fn test_try_state_invalid_accelerator_length() {
    let id = StateID(256);
    let sparse_data = vec![0; 280];
    let transitions = Transitions {
        sparse: sparse_data,
        classes: ByteClasses([0; 256]),
        state_len: 257,
        pattern_len: 1,
    };
    let special = Special {
        max: StateID(256),
        quit_id: StateID(0),
        min_match: StateID(0),
        max_match: StateID(0),
        min_accel: StateID(0),
        max_accel: StateID(0),
        min_start: StateID(0),
        max_start: StateID(0),
    };

    let _ = transitions.try_state(&special, id);
}

