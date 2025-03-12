// Answer 0

#[test]
fn test_fmt_with_unanchored_state_condition() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2)); // This will be the unanchored state
    let state_id_3 = StateID(SmallIndex(3)); // Anchored state

    let state_2 = State {
        id: state_id_2,
        stride2: 0,
        transitions: &[state_id_1.0, state_id_3.0],
    };
    
    let state_3 = State {
        id: state_id_3,
        stride2: 1,
        transitions: &[state_id_1.0],
    };

    let inner = Inner {
        states: vec![state_2.clone(), state_3.clone()],
        start_anchored: state_id_3,
        start_unanchored: state_id_2,
        start_pattern: vec![state_id_2],
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };

    let mut output = String::new();
    let _result = inner.fmt(&mut output);
}

#[test]
fn test_fmt_with_different_sids() {
    let state_id_1 = StateID(SmallIndex(0)); // Start anchored
    let state_id_2 = StateID(SmallIndex(1)); // Start unanchored
    let state_id_3 = StateID(SmallIndex(2)); // Another state
    
    let state_3 = State {
        id: state_id_3,
        stride2: 0,
        transitions: &[state_id_1.0],
    };

    let inner = Inner {
        states: vec![state_3.clone()],
        start_anchored: state_id_1,
        start_unanchored: state_id_2,
        start_pattern: vec![state_id_1],
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };

    let mut output = String::new();
    let _result = inner.fmt(&mut output);
}

