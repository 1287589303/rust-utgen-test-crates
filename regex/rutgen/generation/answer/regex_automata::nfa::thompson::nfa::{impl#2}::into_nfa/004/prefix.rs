// Answer 0

#[test]
fn test_into_nfa_with_capture_state() {
    let capture_state_id = StateID(SmallIndex::new(1));
    
    let state_capture = State::Capture {
        next: StateID(SmallIndex::new(2)),
        pattern_id: PatternID(0),
        group_index: SmallIndex::new(0),
        slot: SmallIndex::new(0),
    };
    
    let state_other = State::ByteRange { trans: Transition::new() };

    let mut inner = Inner {
        states: vec![state_capture.clone(), state_other],
        start_anchored: StateID(SmallIndex::new(0)),
        start_unanchored: StateID(SmallIndex::new(0)),
        start_pattern: vec![StateID(SmallIndex::new(0))],
        ..Default::default()
    };

    let result_nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_multiple_states_in_pattern() {
    let capture_state_id = StateID(SmallIndex::new(1));
    
    let state_capture1 = State::Capture {
        next: StateID(SmallIndex::new(2)),
        pattern_id: PatternID(0),
        group_index: SmallIndex::new(0),
        slot: SmallIndex::new(0),
    };
    
    let state_capture2 = State::Capture {
        next: StateID(SmallIndex::new(3)),
        pattern_id: PatternID(1),
        group_index: SmallIndex::new(1),
        slot: SmallIndex::new(1),
    };

    let state_other = State::ByteRange { trans: Transition::new() };

    let mut inner = Inner {
        states: vec![state_capture1, state_other, state_capture2],
        start_anchored: StateID(SmallIndex::new(0)),
        start_unanchored: StateID(SmallIndex::new(0)),
        start_pattern: vec![StateID(SmallIndex::new(0)), StateID(SmallIndex::new(2))],
        ..Default::default()
    };

    let result_nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_empty_start_pattern() {
    let state_id = StateID(SmallIndex::new(0));
    
    let state_capture = State::Capture {
        next: StateID(SmallIndex::new(1)),
        pattern_id: PatternID(0),
        group_index: SmallIndex::new(0),
        slot: SmallIndex::new(0),
    };
    
    let mut inner = Inner {
        states: vec![state_capture],
        start_anchored: StateID(SmallIndex::new(0)),
        start_unanchored: StateID(SmallIndex::new(0)),
        start_pattern: vec![],
        ..Default::default()
    };

    let result_nfa = inner.into_nfa();
}

