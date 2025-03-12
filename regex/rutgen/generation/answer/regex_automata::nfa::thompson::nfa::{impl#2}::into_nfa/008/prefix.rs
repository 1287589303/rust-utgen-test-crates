// Answer 0

#[test]
fn test_into_nfa_with_dense_state() {
    let state_id = StateID(SmallIndex::new(0));
    let state = State::Dense { transitions: vec![] };
    
    let mut inner = Inner::default();
    inner.states.push(state);
    inner.start_pattern.push(state_id);
    inner.start_anchored = state_id;
    inner.start_unanchored = state_id;
    
    let _nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_byte_range_state() {
    let state_id = StateID(SmallIndex::new(1));
    let input_ranges = [0u8, 255u8];
    let state = State::ByteRange { trans: Transition { input_ranges: &input_ranges } };
    
    let mut inner = Inner::default();
    inner.states.push(state);
    inner.start_pattern.push(state_id);
    inner.start_anchored = state_id;
    inner.start_unanchored = state_id;

    let _nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_fail_state() {
    let state_id = StateID(SmallIndex::new(2));
    let state = State::Fail;
    
    let mut inner = Inner::default();
    inner.states.push(state);
    inner.start_pattern.push(state_id);
    inner.start_anchored = state_id;
    inner.start_unanchored = state_id;

    let _nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_sequential_state_ids() {
    let state_id1 = StateID(SmallIndex::new(3));
    let state_id2 = StateID(SmallIndex::new(4));
    let state = State::Dense { transitions: vec![] };

    let mut inner = Inner::default();
    inner.states.push(state.clone());
    inner.states.push(state.clone());
    inner.start_pattern.push(state_id1);
    inner.start_pattern.push(state_id2);
    inner.start_anchored = state_id1;
    inner.start_unanchored = state_id2;

    let _nfa = inner.into_nfa();
}

