// Answer 0

#[test]
fn test_into_nfa_with_union_state() {
    let start_id_0 = StateID(SmallIndex(0));
    let start_id_1 = StateID(SmallIndex(1));
    
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));
    
    let union_state = State::Union {
        alternates: vec![state_id_2, state_id_3],
    };
    
    let look_state = State::Look {
        look: Look::Start,
        next: state_id_2,
    };
    
    let mut inner = Inner {
        states: vec![union_state.clone(), look_state.clone()],
        start_anchored: start_id_0,
        start_unanchored: start_id_1,
        start_pattern: vec![start_id_0, start_id_1],
        byte_class_set: ByteClassSet::empty(),
        ..Default::default()
    };
    
    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_multiple_unions() {
    let start_id = StateID(SmallIndex(0));
    
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));
    
    let union_1 = State::Union {
        alternates: vec![state_id_1, state_id_2],
    };
    
    let union_2 = State::Union {
        alternates: vec![state_id_2, state_id_3],
    };
    
    let mut inner = Inner {
        states: vec![union_1, union_2],
        start_anchored: start_id,
        start_unanchored: StateID(SmallIndex(1)),
        start_pattern: vec![start_id],
        byte_class_set: ByteClassSet::empty(),
        ..Default::default()
    };
    
    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_union_and_look_states() {
    let start_id = StateID(SmallIndex(0));
    
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let look_state = State::Look {
        look: Look::End,
        next: state_id_2,
    };
    
    let union_state = State::Union {
        alternates: vec![state_id_1, state_id_2],
    };
    
    let mut inner = Inner {
        states: vec![union_state, look_state],
        start_anchored: start_id,
        start_unanchored: StateID(SmallIndex(1)),
        start_pattern: vec![start_id],
        byte_class_set: ByteClassSet::empty(),
        ..Default::default()
    };
    
    let nfa = inner.into_nfa();
}

