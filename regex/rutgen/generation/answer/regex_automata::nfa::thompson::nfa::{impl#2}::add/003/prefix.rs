// Answer 0

#[test]
fn test_add_capture_state() {
    let mut inner = Inner::default();
    let state_id_before = inner.states.len();
    
    let capture_state = State::Capture {
        next: StateID(SmallIndex::new(state_id_before as u32 + 1).unwrap()),
        pattern_id: 0,
        group_index: SmallIndex::new(0).unwrap(),
        slot: SmallIndex::new(0).unwrap(),
    };

    let id = inner.add(capture_state);
}

#[test]
fn test_add_multiple_capture_states() {
    let mut inner = Inner::default();
    
    for i in 0..10 {
        let capture_state = State::Capture {
            next: StateID(SmallIndex::new(i as u32 + 1).unwrap()),
            pattern_id: 0,
            group_index: SmallIndex::new(0).unwrap(),
            slot: SmallIndex::new(0).unwrap(),
        };
        
        let _ = inner.add(capture_state);
    }
}

#[test]
#[should_panic]
fn test_add_capture_state_exceeding_limit() {
    let mut inner = Inner::default();
    let max_states = usize::MAX;
    
    for i in 0..max_states {
        let capture_state = State::Capture {
            next: StateID(SmallIndex::new(i as u32 + 1).unwrap()),
            pattern_id: 0,
            group_index: SmallIndex::new(0).unwrap(),
            slot: SmallIndex::new(0).unwrap(),
        };
        
        let _ = inner.add(capture_state);
    }
    
    let exceeding_capture_state = State::Capture {
        next: StateID(SmallIndex::new(max_states as u32 + 1).unwrap()),
        pattern_id: 0,
        group_index: SmallIndex::new(0).unwrap(),
        slot: SmallIndex::new(0).unwrap(),
    };
    
    let _ = inner.add(exceeding_capture_state);
}

#[test]
fn test_add_capture_state_has_capture_transition() {
    let mut inner = Inner::default();
    assert!(!inner.has_capture);
    
    let capture_state = State::Capture {
        next: StateID(SmallIndex::new(1).unwrap()),
        pattern_id: 0,
        group_index: SmallIndex::new(0).unwrap(),
        slot: SmallIndex::new(0).unwrap(),
    };
    
    let _ = inner.add(capture_state);
    assert!(inner.has_capture);
}

