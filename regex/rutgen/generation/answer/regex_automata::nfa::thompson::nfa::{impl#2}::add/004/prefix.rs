// Answer 0

#[test]
fn test_add_binary_union() {
    let mut inner = Inner::default();
    let state_id_1 = StateID::new(0).unwrap();
    let state_id_2 = StateID::new(1).unwrap();
    
    // Creating a binary union with two state id's
    let binary_union_state = State::BinaryUnion { alt1: state_id_1, alt2: state_id_2 };
    
    // Calling the add method
    let id = inner.add(binary_union_state);
}

#[test]
fn test_add_binary_union_with_capture() {
    let mut inner = Inner::default();
    
    // Adding a capture state as well
    let state_id_capture = StateID::new(2).unwrap();
    let capture_state = State::Capture {
        next: state_id_capture,
        pattern_id: PatternID::new(0).unwrap(),
        group_index: SmallIndex::new(0).unwrap(),
        slot: SmallIndex::new(0).unwrap(),
    };
    
    inner.add(capture_state);
    
    // Now we add a binary union
    let state_id_1 = StateID::new(0).unwrap();
    let state_id_2 = StateID::new(1).unwrap();
    let binary_union_state = State::BinaryUnion { alt1: state_id_1, alt2: state_id_2 };
    
    // Calling the add method
    let id = inner.add(binary_union_state);
}

#[test]
fn test_add_binary_union_boundary_state_limit() {
    let mut inner = Inner::default();
    
    // Filling up the NFA's state limit
    for i in 0..(usize::from(StateID::MAX.0) - 1) {
        let state = State::Fail;
        inner.add(state);
    }
    
    // Now adding the last acceptable state
    let state_id_1 = StateID::new(0).unwrap();
    let state_id_2 = StateID::new(1).unwrap();
    let binary_union_state = State::BinaryUnion { alt1: state_id_1, alt2: state_id_2 };
    
    // Calling the add method
    let id = inner.add(binary_union_state);
}

#[test]
#[should_panic]
fn test_add_binary_union_exceeding_state_limit() {
    let mut inner = Inner::default();
    
    // Filling the NFA to exceed its state limit
    for _ in 0..=usize::from(StateID::MAX.0) {
        let state = State::Fail;
        inner.add(state);
    }

    // This should panic due to exceeding the state limit
    let state_id_1 = StateID::new(0).unwrap();
    let state_id_2 = StateID::new(1).unwrap();
    let binary_union_state = State::BinaryUnion { alt1: state_id_1, alt2: state_id_2 };
    
    // Calling the add method
    let _ = inner.add(binary_union_state);
}

