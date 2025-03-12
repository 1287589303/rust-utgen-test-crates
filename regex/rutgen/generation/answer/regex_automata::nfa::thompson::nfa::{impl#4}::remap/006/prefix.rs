// Answer 0

#[test]
fn test_remap_union_with_valid_alternates() {
    let state_id1 = StateID(SmallIndex(1));
    let state_id2 = StateID(SmallIndex(2));
    let state_id3 = StateID(SmallIndex(3));

    let remap = vec![state_id1, state_id2, state_id3];
    
    let state = State::Union {
        alternates: Box::new([state_id2, state_id3]),
    };

    let mut state = state;
    state.remap(&remap);
}

#[test]
fn test_remap_union_with_single_alternate() {
    let state_id1 = StateID(SmallIndex(1));
    let state_id2 = StateID(SmallIndex(2));

    let remap = vec![state_id1, state_id2];
    
    let state = State::Union {
        alternates: Box::new([state_id1]),
    };

    let mut state = state;
    state.remap(&remap);
}

#[test]
fn test_remap_union_with_no_transition_alternates() {
    let state_id = StateID(SmallIndex(0));

    let remap = vec![state_id];
    
    let state = State::Union {
        alternates: Box::new([state_id]),
    };

    let mut state = state;
    state.remap(&remap);
}

